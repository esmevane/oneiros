use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error("Level not found: {0}")]
    NotFound(LevelName),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

resource_op_error!(LevelError);

impl IntoResponse for LevelError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            LevelError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            LevelError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            LevelError::Database(_) | LevelError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            LevelError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
