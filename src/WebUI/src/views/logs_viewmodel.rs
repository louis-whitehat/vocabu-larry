use serde::Deserialize;

use crate::api::get_json_from_api_base;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    pub files: Vec<String>,
    pub selected_file: Option<String>,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogsViewModel {
    response: Option<LogResponse>,
    error_message: Option<String>,
}

impl LogsViewModel {
    pub fn loading() -> Self {
        Self {
            response: None,
            error_message: None,
        }
    }

    pub fn loaded(response: LogResponse) -> Self {
        Self {
            response: Some(response),
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            response: None,
            error_message: Some(message.into()),
        }
    }

    pub async fn load(file: Option<&str>, api_base: &str) -> Self {
        match fetch_logs(file, api_base).await {
            Ok(response) => Self::loaded(response),
            Err(error) => Self::error(error),
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn is_loading(&self) -> bool {
        self.response.is_none() && self.error_message.is_none()
    }

    pub fn files(&self) -> &[String] {
        self.response
            .as_ref()
            .map(|response| response.files.as_slice())
            .unwrap_or(&[])
    }

    pub fn selected_file(&self) -> Option<&str> {
        self.response
            .as_ref()
            .and_then(|response| response.selected_file.as_deref())
    }

    pub fn content(&self) -> &str {
        self.response
            .as_ref()
            .map(|response| response.content.as_str())
            .unwrap_or("")
    }

    pub fn is_empty(&self) -> bool {
        self.response
            .as_ref()
            .is_some_and(|response| response.files.is_empty())
    }
}

async fn fetch_logs(file: Option<&str>, api_base: &str) -> Result<LogResponse, String> {
    let path = match file {
        Some(file) => format!("/api/logs?file={}", urlencoding::encode(file)),
        None => "/api/logs".to_owned(),
    };

    get_json_from_api_base(api_base, &path).await
}
