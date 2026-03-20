use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum CognitionError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Cognition not found: {0}")]
    NotFound(crate::CognitionId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),
}

impl IntoResponse for CognitionError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            CognitionError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            CognitionError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            CognitionError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            CognitionError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
