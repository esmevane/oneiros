use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, PeerId, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum PeerError {
    #[error("Peer not found: {0}")]
    NotFound(PeerId),

    #[error("Invalid peer address: {0}")]
    InvalidAddress(String),

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

resource_op_error!(PeerError);

impl IntoResponse for PeerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PeerError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            PeerError::InvalidAddress(_) | PeerError::InvalidId(_) | PeerError::Resolve(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            PeerError::Database(_) | PeerError::Event(_) | PeerError::Client(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            PeerError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
