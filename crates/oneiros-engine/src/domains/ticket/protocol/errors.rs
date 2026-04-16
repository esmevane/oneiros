use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("Ticket not found: {0}")]
    NotFound(TicketId),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Brain not found: {0}")]
    BrainNotFound(BrainName),

    #[error("Actor not found: {0}")]
    ActorNotFound(ActorId),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

resource_op_error!(TicketError);

impl IntoResponse for TicketError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TicketError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TicketError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            TicketError::BrainNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TicketError::ActorNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TicketError::Database(_) | TicketError::Event(_) | TicketError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
