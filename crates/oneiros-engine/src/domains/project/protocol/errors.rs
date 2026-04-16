use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("project context required — call start_service first")]
    Missing,

    #[error(transparent)]
    Ticket(#[from] TicketError),

    #[error(transparent)]
    Brain(#[from] BrainError),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

resource_op_error!(ProjectError);

impl IntoResponse for ProjectError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ProjectError::Ticket(ticket) => return ticket.into_response(),
            ProjectError::Brain(brain) => return brain.into_response(),
            ProjectError::Missing
            | ProjectError::Database(_)
            | ProjectError::Event(_)
            | ProjectError::Serde(_)
            | ProjectError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ProjectError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
