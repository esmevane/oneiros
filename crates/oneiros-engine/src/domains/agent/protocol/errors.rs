use crate::*;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    NotFound(AgentName),

    #[error("Agent not found for id: {0}")]
    NotFoundById(AgentId),

    #[error("Persona not found: {0}")]
    PersonaNotFound(PersonaName),

    #[error("Agent already exists: {0}")]
    Conflict(AgentName),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

resource_op_error!(AgentError);

impl IntoResponse for AgentError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AgentError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AgentError::NotFoundById(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AgentError::PersonaNotFound(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AgentError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            AgentError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AgentError::Database(_) | AgentError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AgentError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
