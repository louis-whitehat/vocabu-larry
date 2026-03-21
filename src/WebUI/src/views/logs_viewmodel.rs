use std::future::Future;

use serde::Deserialize;

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

    pub async fn load_with<F, Fut>(loader: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<LogResponse, String>>,
    {
        match loader().await {
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