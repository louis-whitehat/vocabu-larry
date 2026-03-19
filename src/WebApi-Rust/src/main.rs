use std::{
    collections::BTreeMap,
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use chrono::Local;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs, sync::Mutex};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

const DEFAULT_HTTP_PORT: u16 = 8101;
const DEFAULT_HTTPS_PORT: u16 = 8102;

#[derive(Clone)]
struct AppState {
    home_dir: PathBuf,
    score_lock: Arc<Mutex<()>>,
}

#[derive(Debug, Serialize)]
struct UserEntry {
    name: String,
    dictionaries: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DictionaryQuery {
    user: String,
    dictionary: String,
}

#[derive(Debug, Deserialize)]
struct ScoreQuery {
    user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScoreRequest {
    user: String,
    dictionary: String,
    is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ScoreEntry {
    total: u64,
    correct: u64,
}

type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;

#[derive(Debug, Error)]
enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("internal server error")]
    Internal(#[from] std::io::Error),
    #[error("invalid json")]
    InvalidJson(#[from] serde_json::Error),
}

impl AppError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidConfiguration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) | Self::InvalidJson(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        error!(error = %self, "request failed");
        (status, self.to_string()).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vocabu_larry_api=info,tower_http=info".into()),
        )
        .init();

    let home_dir = resolve_home_dir()?;
    let state = AppState {
        home_dir,
        score_lock: Arc::new(Mutex::new(())),
    };
    let tls_home_dir = state.home_dir.clone();
    let http_port = read_port("VOCABULARRY_HTTP_PORT", DEFAULT_HTTP_PORT)?;
    let https_port = read_port("VOCABULARRY_HTTPS_PORT", DEFAULT_HTTPS_PORT)?;

    let app = Router::new()
        .route("/api/users", get(get_users))
        .route("/api/dictionary", get(get_dictionary))
        .route("/api/score", get(get_score).post(post_score))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let http_addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    let https_addr = SocketAddr::from(([0, 0, 0, 0], https_port));

    let http_listener = tokio::net::TcpListener::bind(http_addr)
        .await
        .map_err(AppError::from)?;

    info!(address = %http_addr, "listening for HTTP traffic");

    if let Some(tls_config) = load_tls_config(&tls_home_dir).await? {
        info!(address = %https_addr, "listening for HTTPS traffic");

        let http_server = axum::serve(http_listener, app.clone().into_make_service());
        let https_server = axum_server::bind_rustls(https_addr, tls_config)
            .serve(app.into_make_service());

        tokio::try_join!(http_server, https_server).map_err(AppError::from)?;
    } else {
        info!(address = %https_addr, "HTTPS disabled because certificates are missing");
        axum::serve(http_listener, app.into_make_service())
            .await
            .map_err(AppError::from)?;
    }

    Ok(())
}

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserEntry>>, AppError> {
    let dictionaries_root = state.home_dir.join("dictionaries");
    let mut users = Vec::new();
    let mut entries = fs::read_dir(&dictionaries_root).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        let mut dictionaries = list_dictionary_names(&entry.path()).await?;
        dictionaries.sort();

        users.push(UserEntry { name, dictionaries });
    }

    users.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(Json(users))
}

async fn get_dictionary(
    State(state): State<AppState>,
    Query(query): Query<DictionaryQuery>,
) -> Result<String, AppError> {
    let file = dictionary_file_path(&state.home_dir, &query.user, &query.dictionary)?;

    match fs::read_to_string(&file).await {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(AppError::not_found("dictionary does not exist"))
        }
        Err(error) => Err(AppError::from(error)),
    }
}

async fn get_score(
    State(state): State<AppState>,
    Query(query): Query<ScoreQuery>,
) -> Result<Json<ScoreStore>, AppError> {
    let user = validate_path_segment(&query.user, "user")?;
    let file = state.home_dir.join(format!("score-{user}.json"));

    if !fs::try_exists(&file).await? {
        return Ok(Json(ScoreStore::default()));
    }

    Ok(Json(read_json_file(&file).await?))
}

async fn post_score(
    State(state): State<AppState>,
    Json(payload): Json<ScoreRequest>,
) -> Result<&'static str, AppError> {
    let user = validate_path_segment(&payload.user, "user")?;
    let dictionary = validate_path_segment(&payload.dictionary, "dictionary")?;
    let file = state.home_dir.join(format!("score-{user}.json"));
    let date = Local::now().format("%Y-%m-%d").to_string();

    let _guard = state.score_lock.lock().await;

    let mut store = if fs::try_exists(&file).await? {
        read_json_file(&file).await?
    } else {
        ScoreStore::default()
    };

    let entry = store
        .entry(date)
        .or_default()
        .entry(dictionary.to_owned())
        .or_default();

    entry.total += 1;
    if payload.is_correct {
        entry.correct += 1;
    }

    let content = serde_json::to_vec(&store)?;
    fs::write(file, content).await?;

    Ok("ok")
}

async fn list_dictionary_names(user_dir: &Path) -> Result<Vec<String>, AppError> {
    let mut names = Vec::new();
    let mut entries = fs::read_dir(user_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_file() {
            continue;
        }

        if let Some(name) = entry.path().file_stem().and_then(|value| value.to_str()) {
            names.push(name.to_owned());
        }
    }

    Ok(names)
}

async fn read_json_file<T>(file: &Path) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(file).await?;
    Ok(serde_json::from_str(&content)?)
}

fn resolve_home_dir() -> Result<PathBuf, AppError> {
    let raw_home = if is_production() {
        env::var("VOCABULARRY_HOME").map_err(|_| {
            AppError::bad_request("VOCABULARRY_HOME must be set when NODE_ENV=production")
        })?
    } else {
        "../../".to_owned()
    };

    Ok(PathBuf::from(raw_home))
}

fn is_production() -> bool {
    matches!(env::var("NODE_ENV").as_deref(), Ok("production"))
}

fn read_port(name: &str, default: u16) -> Result<u16, AppError> {
    match env::var(name) {
        Ok(value) => value.parse::<u16>().map_err(|_| {
            AppError::InvalidConfiguration(format!("{name} must be a valid port number"))
        }),
        Err(_) => Ok(default),
    }
}

async fn load_tls_config(home_dir: &Path) -> Result<Option<RustlsConfig>, AppError> {
    let cert_path = home_dir.join("selfsigned.crt");
    let key_path = home_dir.join("selfsigned.key");

    if !fs::try_exists(&cert_path).await? || !fs::try_exists(&key_path).await? {
        return Ok(None);
    }

    Ok(Some(
        RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .map_err(|error| AppError::bad_request(error.to_string()))?,
    ))
}

fn dictionary_file_path(home_dir: &Path, user: &str, dictionary: &str) -> Result<PathBuf, AppError> {
    let user = validate_path_segment(user, "user")?;
    let dictionary = validate_path_segment(dictionary, "dictionary")?;

    Ok(home_dir
        .join("dictionaries")
        .join(user)
        .join(format!("{dictionary}.txt")))
}

fn validate_path_segment<'a>(value: &'a str, field: &str) -> Result<&'a str, AppError> {
    if value.trim().is_empty() {
        return Err(AppError::bad_request(format!("{field} must not be empty")));
    }

    let path = Path::new(value);
    let is_single_segment = path.components().count() == 1;
    let exact_name_match = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == value);

    if is_single_segment && exact_name_match {
        Ok(value)
    } else {
        Err(AppError::bad_request(format!("invalid {field}")))
    }
}