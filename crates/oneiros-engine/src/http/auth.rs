use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    NoAuthHeader,
    #[error("Invalid authorization header")]
    InvalidAuthHeader,
    #[error("Invalid or expired token")]
    InvalidToken,
    #[error("Brain not found: {0}")]
    BrainNotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::NoAuthHeader | AuthError::InvalidAuthHeader | AuthError::InvalidToken => {
                StatusCode::UNAUTHORIZED
            }
            AuthError::BrainNotFound(_) => StatusCode::NOT_FOUND,
            AuthError::Database(_) | AuthError::Internal(_) | AuthError::Event(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}
