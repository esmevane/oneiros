use axum::response::{IntoResponse, Response};

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
        let status = match &self {
            McpConfigError::NoToken => axum::http::StatusCode::PRECONDITION_FAILED,
            McpConfigError::Io(_) | McpConfigError::Json(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, self.to_string()).into_response()
    }
}
