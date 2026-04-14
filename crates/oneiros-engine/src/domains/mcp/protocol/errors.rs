use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum McpConfigError {
    #[error("No token available. Run `oneiros project init` first, or pass --token.")]
    NoToken,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for McpConfigError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            McpConfigError::NoToken => (StatusCode::PRECONDITION_FAILED, self.to_string()),
            McpConfigError::Io(_) | McpConfigError::Json(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
