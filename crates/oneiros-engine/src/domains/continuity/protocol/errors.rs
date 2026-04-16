use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ContinuityError {
    #[error("Agent not found: {0}")]
    AgentNotFound(AgentName),

    #[error(transparent)]
    Agent(#[from] AgentError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),

    #[error("Unexpected service response: {0}")]
    UnexpectedResponse(String),
}

resource_op_error!(ContinuityError);

impl IntoResponse for ContinuityError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ContinuityError::AgentNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ContinuityError::Agent(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ContinuityError::Database(_) | ContinuityError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ContinuityError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            ContinuityError::UnexpectedResponse(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
