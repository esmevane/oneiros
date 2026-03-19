use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Connection not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),
}

impl IntoResponse for ConnectionError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ConnectionError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ConnectionError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
