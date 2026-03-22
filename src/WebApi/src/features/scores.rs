use crate::{error::AppError, shared::validate_path_segment, state::AppState};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tokio::fs;

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
    let file = state.home_dir.join(format!("score-{user}.json"));

    if !fs::try_exists(&file).await? {
        return Ok(Json(ScoreStore::default()));
    }

    let content = fs::read_to_string(&file).await?;
    Ok(Json(serde_json::from_str(&content)?))
}

pub async fn post_score(
    State(state): State<AppState>,
    Json(payload): Json<ScoreRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = validate_path_segment(&payload.user, "user")?;
    let dictionary = validate_path_segment(&payload.dictionary, "dictionary")?;
    let file = state.home_dir.join(format!("score-{user}.json"));
    let date = Local::now().format("%Y-%m-%d").to_string();

    let _guard = state.score_lock.lock().await;

    let mut store = if fs::try_exists(&file).await? {
        let content = fs::read_to_string(&file).await?;
        serde_json::from_str(&content)?
    } else {
        ScoreStore::default()
    };

    let entry = store.entry(date).or_insert_with(BTreeMap::new).entry(dictionary.to_owned()).or_default();

    entry.total += 1;
    if payload.is_correct {
        entry.correct += 1;
    }

    let content = serde_json::to_vec(&store)?;
    fs::write(file, content).await?;

    Ok("ok")
}
