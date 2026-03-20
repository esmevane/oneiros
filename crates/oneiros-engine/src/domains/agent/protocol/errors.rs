use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    NotFound(AgentName),

    #[error("Persona not found: {0}")]
    PersonaNotFound(PersonaName),

    #[error("Agent already exists: {0}")]
    Conflict(AgentName),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),
}

impl IntoResponse for AgentError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AgentError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AgentError::PersonaNotFound(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AgentError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            AgentError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
