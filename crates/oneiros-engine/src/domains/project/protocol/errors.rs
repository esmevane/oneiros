use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProjectError {
    #[error("project context required — call start_service first")]
    Missing,

    #[error("Project not found: {0}")]
    NotFound(ProjectName),

    #[error("Project not found for id: {0}")]
    NotFoundById(ProjectId),

    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error(transparent)]
    Ticket(#[from] TicketError),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Compose(#[from] crate::ComposeError),

    #[error(transparent)]
    BookmarkDb(#[from] crate::BookmarkDbError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error(transparent)]
    Upcast(#[from] UpcastError),
}

resource_op_error!(ProjectError);

impl IntoResponse for ProjectError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ProjectError::Ticket(ticket) => return ticket.into_response(),
            ProjectError::NotFound(_) | ProjectError::NotFoundById(_) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            ProjectError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ProjectError::Missing
            | ProjectError::Database(_)
            | ProjectError::Event(_)
            | ProjectError::Io(_)
            | ProjectError::Upcast(_)
            | ProjectError::Compose(_)
            | ProjectError::BookmarkDb(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ProjectError::Client(_) | ProjectError::Serde(_) => {
                (StatusCode::BAD_GATEWAY, self.to_string())
            }
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
