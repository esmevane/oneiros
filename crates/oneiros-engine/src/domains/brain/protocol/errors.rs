use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{BrainName, ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum BrainError {
    #[error("Brain not found: {0}")]
    NotFound(BrainName),

    #[error("Brain not found for id: {0}")]
    NotFoundById(crate::BrainId),

    #[error("Brain already exists: {0}")]
    Conflict(BrainName),

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

resource_op_error!(BrainError);

impl IntoResponse for BrainError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            BrainError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BrainError::NotFoundById(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BrainError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            BrainError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            BrainError::Database(_) | BrainError::Event(_) | BrainError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            BrainError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
