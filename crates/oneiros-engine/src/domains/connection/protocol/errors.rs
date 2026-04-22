use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Connection not found: {0}")]
    NotFound(ConnectionId),

    #[error("Invalid entity reference: {0}")]
    InvalidRef(String),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(ConnectionError);

impl IntoResponse for ConnectionError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ConnectionError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ConnectionError::InvalidRef(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ConnectionError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ConnectionError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ConnectionError::Database(_) | ConnectionError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ConnectionError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
