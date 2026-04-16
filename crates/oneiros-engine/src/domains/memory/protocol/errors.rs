use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Memory not found: {0}")]
    NotFound(crate::MemoryId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(MemoryError);

impl IntoResponse for MemoryError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            MemoryError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            MemoryError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            MemoryError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            MemoryError::Database(_) | MemoryError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            MemoryError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
