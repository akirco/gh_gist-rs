use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GistError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API error (status {status}): {message}")]
    ApiError { status: StatusCode, message: String },

    #[error("Authentication required")]
    Unauthorized,

    #[error("Resource not found")]
    NotFound,

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("File operation error: {0}")]
    FileError(String),
}

impl GistError {
    pub fn from_response(status: StatusCode, message: String) -> Self {
        match status {
            StatusCode::UNAUTHORIZED => GistError::Unauthorized,
            StatusCode::NOT_FOUND => GistError::NotFound,
            _ => GistError::ApiError { status, message },
        }
    }
}
