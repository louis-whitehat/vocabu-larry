use std::path::Path;

use chrono::Local;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{error::AppError, state::AppState};

pub async fn append_request_error(
    state: &AppState,
    method: &str,
    path: &str,
    status_code: u16,
    message: &str,
) -> Result<(), AppError> {
    let timestamp = Local::now();
    let trimmed_message = message.trim();
    let entry = if trimmed_message.is_empty() {
        format!("[{}] {} {} {}\n\n", timestamp.format("%Y-%m-%d %H:%M:%S"), method, path, status_code)
    } else {
        format!(
            "[{}] {} {} {}\n{}\n\n",
            timestamp.format("%Y-%m-%d %H:%M:%S"),
            method,
            path,
            status_code,
            trimmed_message
        )
    };

    append_entry(state, entry).await
}

pub async fn append_login_event(state: &AppState, user: &str) -> Result<(), AppError> {
    let timestamp = Local::now();
    let entry = format!("[{}] LOGIN user={}\n\n", timestamp.format("%Y-%m-%d %H:%M:%S"), user);

    append_entry(state, entry).await
}

async fn append_entry(state: &AppState, entry: String) -> Result<(), AppError> {
    let timestamp = Local::now();
    let file_name = format!("{}.log", timestamp.format("%Y-%m-%d"));
    let file_path = state.log_dir.join(file_name);

    let _guard = state.log_lock.lock().await;
    fs::create_dir_all(&state.log_dir).await?;

    let mut file = OpenOptions::new().create(true).append(true).open(file_path).await?;
    file.write_all(entry.as_bytes()).await?;

    Ok(())
}

pub async fn list_log_files(log_dir: &Path) -> Result<Vec<String>, AppError> {
    if !fs::try_exists(log_dir).await? {
        return Ok(Vec::new());
    }

    let mut entries = fs::read_dir(log_dir).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_file() {
            continue;
        }

        let Some(file_name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };

        if file_name.ends_with(".log") {
            files.push(file_name);
        }
    }

    files.sort_by(|left, right| right.cmp(left));

    Ok(files)
}

pub async fn read_log_file(log_dir: &Path, file_name: &str) -> Result<String, AppError> {
    let file_path = log_dir.join(file_name);

    match fs::read_to_string(&file_path).await {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(AppError::new("log file does not exist")),
        Err(error) => Err(AppError::from(error)),
    }
}
