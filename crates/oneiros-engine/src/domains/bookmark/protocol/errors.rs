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
            BookmarkError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BookmarkError::AlreadyExists(_) => (StatusCode::CONFLICT, self.to_string()),
            BookmarkError::Database(_)
            | BookmarkError::Event(_)
            | BookmarkError::IdParse(_)
            | BookmarkError::TimestampParse(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            BookmarkError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
