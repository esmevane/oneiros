use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{ErrorResponse, UpcastError};

#[derive(Debug, thiserror::Error)]
pub enum McpConfigError {
    #[error("No token available. Run `oneiros project init` first, or pass --token.")]
    NoToken,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Upcast(#[from] UpcastError),
}

impl IntoResponse for McpConfigError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            McpConfigError::NoToken => (StatusCode::PRECONDITION_FAILED, self.to_string()),
            McpConfigError::Io(_) | McpConfigError::Json(_) | McpConfigError::Upcast(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
