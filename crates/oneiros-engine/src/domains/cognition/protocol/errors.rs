use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum CognitionError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Cognition not found: {0}")]
    NotFound(crate::CognitionId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for CognitionError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            CognitionError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            CognitionError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            CognitionError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            CognitionError::Database(_) | CognitionError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            CognitionError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
