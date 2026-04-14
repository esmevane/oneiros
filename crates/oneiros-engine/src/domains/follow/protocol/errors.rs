use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, FollowId};

#[derive(Debug, thiserror::Error)]
pub enum FollowError {
    #[error("Follow not found: {0}")]
    NotFound(FollowId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for FollowError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            FollowError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            FollowError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            FollowError::Database(_) | FollowError::Event(_) | FollowError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
