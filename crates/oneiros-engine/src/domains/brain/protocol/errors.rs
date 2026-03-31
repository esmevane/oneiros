use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::BrainName;

#[derive(Debug, thiserror::Error)]
pub enum BrainError {
    #[error("Brain not found: {0}")]
    NotFound(BrainName),

    #[error("Brain already exists: {0}")]
    Conflict(BrainName),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for BrainError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            BrainError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BrainError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            BrainError::Database(_) | BrainError::Event(_) | BrainError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
