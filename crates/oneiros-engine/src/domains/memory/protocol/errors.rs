use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Memory not found: {0}")]
    NotFound(crate::MemoryId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),
}

impl IntoResponse for MemoryError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            MemoryError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            MemoryError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            MemoryError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            MemoryError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
