use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::ActorId;

#[derive(Debug, thiserror::Error)]
pub enum ActorError {
    #[error("Actor not found: {0}")]
    NotFound(ActorId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for ActorError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ActorError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ActorError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ActorError::Database(_) | ActorError::Event(_) | ActorError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
