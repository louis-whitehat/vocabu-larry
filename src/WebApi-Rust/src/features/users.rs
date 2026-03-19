use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use tokio::fs;

use crate::{error::AppError, features::training, state::AppState};

#[derive(Debug, Serialize)]
pub struct UserEntry {
    name: String,
    dictionaries: Vec<String>,
}

pub async fn get_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let dictionaries_root = state.home_dir.join("dictionaries");
    let mut users = Vec::new();
    let mut entries = fs::read_dir(&dictionaries_root).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        let mut dictionaries = training::list_dictionary_names(&entry.path()).await?;
        dictionaries.sort();

        users.push(UserEntry { name, dictionaries });
    }

    users.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(Json(users))
}