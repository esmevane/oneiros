use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error(transparent)]
    Client(#[from] ClientError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error("Required personas (process, scribe) not found. Run `oneiros seed core` first.")]
    MissingPersonas,
}

impl IntoResponse for SeedError {
    fn into_response(self) -> Response {
        match self {
            SeedError::Client(_) => (
                axum::http::StatusCode::BAD_GATEWAY,
                self.to_string(),
            )
                .into_response(),
            SeedError::Event(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            )
                .into_response(),
            SeedError::MissingPersonas => (
                axum::http::StatusCode::PRECONDITION_FAILED,
                self.to_string(),
            )
                .into_response(),
        }
    }
}
