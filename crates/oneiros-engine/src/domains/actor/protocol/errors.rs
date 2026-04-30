use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ActorError {
    #[error("Actor not found: {0}")]
    NotFound(ActorId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Compose(#[from] crate::ComposeError),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(ActorError);

impl IntoResponse for ActorError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ActorError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ActorError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ActorError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ActorError::Database(_)
            | ActorError::Event(_)
            | ActorError::Client(_)
            | ActorError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
