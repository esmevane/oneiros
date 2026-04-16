use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum ExperienceError {
    #[error("Agent not found: {0}")]
    AgentNotFound(crate::AgentName),

    #[error("Experience not found: {0}")]
    NotFound(crate::ExperienceId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error("{0}")]
    InvalidRequest(String),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(ExperienceError);

impl IntoResponse for ExperienceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ExperienceError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ExperienceError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ExperienceError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ExperienceError::Database(_) | ExperienceError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ExperienceError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ExperienceError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
