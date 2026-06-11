use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum PeerError {
    #[error("Peer not found: {0}")]
    NotFound(PeerId),

    #[error(transparent)]
    ParseAddressFailed(#[from] PeerAddressError),

    #[error("Invalid peer address: must be an oneiros:// link")]
    InvalidAddress,

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] IdParseError),

    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Compose(#[from] ComposeError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("Only bookmarks and projects permitted for peering")]
    InvalidRef,
}

resource_op_error!(PeerError);

impl IntoResponse for PeerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PeerError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            PeerError::InvalidAddress
            | PeerError::ParseAddressFailed(_)
            | PeerError::InvalidRef
            | PeerError::InvalidId(_)
            | PeerError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            PeerError::Database(_) | PeerError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            PeerError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            PeerError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            PeerError::Json(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
