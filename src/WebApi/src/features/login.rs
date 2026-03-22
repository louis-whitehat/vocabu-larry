use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{error::AppError, logging, shared::validate_path_segment, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    user: String,
}

pub async fn post_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = validate_path_segment(&payload.user, "user")?;

    logging::append_login_event(&state, user).await?;

    Ok("ok")
}