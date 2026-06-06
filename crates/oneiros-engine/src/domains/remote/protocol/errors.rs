use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

resource_op_error!(RemoteError);

#[derive(Debug, thiserror::Error)]
pub(crate) enum RemoteError {
    #[error("remote not found: {0}")]
    NotFound(RemoteName),

    #[error("remote already exists: {0}")]
    AlreadyExists(RemoteName),

    #[error("invalid ticket URI: {0}")]
    InvalidTicket(String),

    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    HostDb(#[from] HostDbError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    IdParse(#[from] IdParseError),

    #[error(transparent)]
    Bridge(#[from] BridgeError),

    #[error(transparent)]
    Ticket(#[from] TicketError),

    #[error(transparent)]
    Bookmark(#[from] BookmarkError),

    #[error(transparent)]
    PeerLink(#[from] PeerLinkError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Scope(#[from] ScopeError),

    #[error(transparent)]
    Compose(#[from] ComposeError),
}

impl IntoResponse for RemoteError {
    fn into_response(self) -> Response {
        let status = match &self {
            RemoteError::NotFound(_) => StatusCode::NOT_FOUND,
            RemoteError::AlreadyExists(_) => StatusCode::CONFLICT,
            RemoteError::InvalidTicket(_) | RemoteError::ConnectionFailed(_) => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
