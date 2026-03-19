use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    error::AppError,
    state::AppState,
    storage,
    types::{DictionaryQuery, ScoreQuery, ScoreRequest, ScoreStore, UserEntry},
};

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserEntry>>, AppError> {
    Ok(Json(storage::list_users(&state.home_dir).await?))
}

pub async fn get_dictionary(
    State(state): State<AppState>,
    Query(query): Query<DictionaryQuery>,
) -> Result<String, AppError> {
    storage::read_dictionary(&state.home_dir, &query.user, &query.dictionary).await
}

pub async fn get_score(
    State(state): State<AppState>,
    Query(query): Query<ScoreQuery>,
) -> Result<Json<ScoreStore>, AppError> {
    Ok(Json(storage::read_score_store(&state.home_dir, &query.user).await?))
}

pub async fn post_score(
    State(state): State<AppState>,
    Json(payload): Json<ScoreRequest>,
) -> Result<&'static str, AppError> {
    storage::record_score(
        &state.home_dir,
        &state.score_lock,
        &payload.user,
        &payload.dictionary,
        payload.is_correct,
    )
    .await?;

    Ok("ok")
}