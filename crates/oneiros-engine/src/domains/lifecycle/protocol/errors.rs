use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum LifecycleError {
    #[error("Agent not found: {0}")]
    AgentNotFound(AgentName),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for LifecycleError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            LifecycleError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            LifecycleError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            LifecycleError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
