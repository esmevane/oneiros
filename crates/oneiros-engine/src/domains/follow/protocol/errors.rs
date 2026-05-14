use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, FollowId, ResolveError, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub(crate) enum FollowError {
    #[error("Follow not found: {0}")]
    NotFound(FollowId),

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

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for FollowError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            FollowError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                ErrorResponse::new(self.to_string()).with_code("not_found"),
            ),
            FollowError::InvalidId(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::new(self.to_string()).with_code("invalid_id"),
            ),
            FollowError::Resolve(ResolveError::WrongKind { expected, got }) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::new(self.to_string())
                    .with_code("wrong_kind")
                    .with_detail(format!("expected a {expected} ref, got a {got} ref")),
            ),
            FollowError::Database(_) | FollowError::Event(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(self.to_string()),
            ),
            FollowError::Client(_) | FollowError::Json(_) => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse::new(self.to_string()),
            ),
            FollowError::Compose(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(self.to_string()),
            ),
        };
        (status, Json(body)).into_response()
    }
}

resource_op_error!(FollowError);
