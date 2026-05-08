use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AuthError {
    #[error("Missing authorization header")]
    NoAuthHeader,
    #[error("Invalid or expired token")]
    InvalidToken,
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    #[error(transparent)]
    Event(#[from] EventError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::NoAuthHeader | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::Database(_) | AuthError::Event(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, axum::Json(ErrorResponse::new(self.to_string()))).into_response()
    }
}
