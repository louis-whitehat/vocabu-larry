use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::Local;
use serde::de::DeserializeOwned;
use tokio::{fs, sync::Mutex};

use crate::{
    error::AppError,
    types::{ScoreStore, UserEntry},
};

pub async fn list_users(home_dir: &Path) -> Result<Vec<UserEntry>, AppError> {
    let dictionaries_root = home_dir.join("dictionaries");
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

    Ok(users)
}

pub async fn read_dictionary(
    home_dir: &Path,
    user: &str,
    dictionary: &str,
) -> Result<String, AppError> {
    let file = dictionary_file_path(home_dir, user, dictionary)?;

    match fs::read_to_string(&file).await {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(AppError::not_found("dictionary does not exist"))
        }
        Err(error) => Err(AppError::from(error)),
    }
}

pub async fn read_score_store(home_dir: &Path, user: &str) -> Result<ScoreStore, AppError> {
    let user = validate_path_segment(user, "user")?;
    let file = score_file_path(home_dir, user);

    if !fs::try_exists(&file).await? {
        return Ok(ScoreStore::default());
    }

    read_json_file(&file).await
}

pub async fn record_score(
    home_dir: &Path,
    score_lock: &Arc<Mutex<()>>,
    user: &str,
    dictionary: &str,
    is_correct: bool,
) -> Result<(), AppError> {
    let user = validate_path_segment(user, "user")?;
    let dictionary = validate_path_segment(dictionary, "dictionary")?;
    let file = score_file_path(home_dir, user);
    let date = current_date();

    let _guard = score_lock.lock().await;

    let mut store = if fs::try_exists(&file).await? {
        read_json_file(&file).await?
    } else {
        ScoreStore::default()
    };

    let entry = store
        .entry(date)
        .or_insert_with(BTreeMap::new)
        .entry(dictionary.to_owned())
        .or_default();

    entry.total += 1;
    if is_correct {
        entry.correct += 1;
    }

    let content = serde_json::to_vec(&store)?;
    fs::write(file, content).await?;

    Ok(())
}

fn current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn score_file_path(home_dir: &Path, user: &str) -> PathBuf {
    home_dir.join(format!("score-{user}.json"))
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