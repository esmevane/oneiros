use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ExperienceError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Experience not found: {0}")]
    NotFound(crate::ExperienceId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),

    #[error("{0}")]
    InvalidRequest(String),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for ExperienceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ExperienceError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ExperienceError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ExperienceError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ExperienceError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ExperienceError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ExperienceError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
