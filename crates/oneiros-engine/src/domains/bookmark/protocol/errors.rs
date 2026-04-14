use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum BookmarkError {
    #[error("Bookmark not found: {0}")]
    NotFound(BookmarkName),

    #[error("Bookmark already exists: {0}")]
    AlreadyExists(BookmarkName),

    #[error("Brain not found: {0}")]
    BrainNotFound(BrainName),

    #[error("Actor not found: {0}")]
    ActorNotFound(ActorId),

    #[error("No actor available — provide an actor_id or initialize the system first")]
    NoActor,

    #[error("Invalid URI: {0}")]
    InvalidUri(String),

    #[error("Follow not found for bookmark: {0}")]
    FollowNotFound(BookmarkName),

    #[error(transparent)]
    Follow(#[from] FollowError),

    #[error(transparent)]
    Peer(#[from] PeerError),

    #[error(transparent)]
    Ticket(#[from] TicketError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    IdParse(#[from] IdParseError),

    #[error(transparent)]
    TimestampParse(#[from] TimestampParseError),
}

impl IntoResponse for BookmarkError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            BookmarkError::NotFound(_)
            | BookmarkError::BrainNotFound(_)
            | BookmarkError::ActorNotFound(_)
            | BookmarkError::FollowNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BookmarkError::AlreadyExists(_) => (StatusCode::CONFLICT, self.to_string()),
            BookmarkError::InvalidUri(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            BookmarkError::NoActor
            | BookmarkError::Database(_)
            | BookmarkError::Event(_)
            | BookmarkError::IdParse(_)
            | BookmarkError::TimestampParse(_)
            | BookmarkError::Follow(_)
            | BookmarkError::Peer(_)
            | BookmarkError::Ticket(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            BookmarkError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
