use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, logging, shared::validate_path_segment, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LogQuery {
    file: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    files: Vec<String>,
    selected_file: Option<String>,
    content: String,
}

pub async fn get_logs(
    State(state): State<AppState>,
    Query(query): Query<LogQuery>,
) -> Result<impl IntoResponse, AppError> {
    let files = logging::list_log_files(&state.log_dir).await?;
    let selected_file = match query.file {
        Some(file) => Some(validate_path_segment(&file, "file")?.to_owned()),
        None => files.first().cloned(),
    };

    let content = match &selected_file {
        Some(file) => logging::read_log_file(&state.log_dir, file).await?,
        None => String::new(),
    };

    Ok(Json(LogResponse { files, selected_file, content }))
}