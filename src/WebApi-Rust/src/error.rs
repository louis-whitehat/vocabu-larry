use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InvalidConfiguration(String),
    Internal(String),
    InvalidJson(String),
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(message) => write!(formatter, "bad request: {message}"),
            Self::NotFound(message) => write!(formatter, "not found: {message}"),
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid configuration: {message}")
            }
            Self::Internal(message) => write!(formatter, "internal server error: {message}"),
            Self::InvalidJson(message) => write!(formatter, "invalid json: {message}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::InvalidJson(error.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidConfiguration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) | Self::InvalidJson(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        eprintln!("request failed: {self}");
        (status, self.to_string()).into_response()
    }
}
