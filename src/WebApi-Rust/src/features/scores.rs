use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path, sync::Arc};
use tokio::{fs, sync::Mutex};

use crate::{error::AppError, shared::validate_path_segment, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ScoreQuery {
    user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreRequest {
    user: String,
    dictionary: String,
    is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScoreEntry {
    pub total: u64,
    pub correct: u64,
}

pub type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;

pub async fn get_score(
    State(state): State<AppState>,
    Query(query): Query<ScoreQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user = validate_path_segment(&query.user, "user")?;
    let file = score_file_path(&state.home_dir, user);

    if !fs::try_exists(&file).await? {
        return Ok(Json(ScoreStore::default()));
    }

    Ok(Json(read_json_file(&file).await?))
}

pub async fn post_score(
    State(state): State<AppState>,
    Json(payload): Json<ScoreRequest>,
) -> Result<impl IntoResponse, AppError> {
    record_score(&state.home_dir, &state.score_lock, &payload.user, &payload.dictionary, payload.is_correct).await?;

    Ok("ok")
}

async fn record_score(
    home_dir: &Path,
    score_lock: &Arc<Mutex<()>>,
    user: &str,
    dictionary: &str,
    is_correct: bool,
) -> Result<(), AppError> {
    let user = validate_path_segment(user, "user")?;
    let dictionary = validate_path_segment(dictionary, "dictionary")?;
    let file = score_file_path(home_dir, user);
    let date = Local::now().format("%Y-%m-%d").to_string();

    let _guard = score_lock.lock().await;

    let mut store = if fs::try_exists(&file).await? { read_json_file(&file).await? } else { ScoreStore::default() };

    let entry = store.entry(date).or_insert_with(BTreeMap::new).entry(dictionary.to_owned()).or_default();

    entry.total += 1;
    if is_correct {
        entry.correct += 1;
    }

    let content = serde_json::to_vec(&store)?;
    fs::write(file, content).await?;

    Ok(())
}

fn score_file_path(home_dir: &Path, user: &str) -> std::path::PathBuf {
    home_dir.join(format!("score-{user}.json"))
}

async fn read_json_file<T>(file: &Path) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(file).await?;
    Ok(serde_json::from_str(&content)?)
}
