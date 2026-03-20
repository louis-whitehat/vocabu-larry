use std::path::{Path, PathBuf};

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{error::AppError, shared::validate_path_segment, state::AppState};

#[derive(Debug, Deserialize)]
pub struct DictionaryQuery {
    user: String,
    dictionary: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct DictionaryEntry {
    word: String,
    translation: String,
}

pub async fn get_dictionary(
    State(state): State<AppState>,
    Query(query): Query<DictionaryQuery>,
) -> Result<Json<Vec<DictionaryEntry>>, AppError> {
    let file = dictionary_file_path(&state.home_dir, &query.user, &query.dictionary)?;

    match fs::read_to_string(&file).await {
        Ok(content) => Ok(Json(parse_dictionary_content(&content)?)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(AppError::new("dictionary does not exist")),
        Err(error) => Err(AppError::from(error)),
    }
}

pub async fn list_dictionary_names(user_dir: &Path) -> Result<Vec<String>, AppError> {
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

fn dictionary_file_path(home_dir: &Path, user: &str, dictionary: &str) -> Result<PathBuf, AppError> {
    let user = validate_path_segment(user, "user")?;
    let dictionary = validate_path_segment(dictionary, "dictionary")?;

    Ok(home_dir.join("dictionaries").join(user).join(format!("{dictionary}.txt")))
}

fn parse_dictionary_content(content: &str) -> Result<Vec<DictionaryEntry>, AppError> {
    let mut entries = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let (word, translation) = line
            .split_once(':')
            .ok_or_else(|| AppError::new(format!("dictionary line {} is missing ':'", line_number + 1)))?;
        let word = normalize_whitespace(word);
        let translation = normalize_whitespace(translation);

        if word.is_empty() || translation.is_empty() {
            return Err(AppError::new(format!(
                "dictionary line {} must contain both word and translation",
                line_number + 1
            )));
        }

        entries.push(DictionaryEntry { word, translation });
    }

    Ok(entries)
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
