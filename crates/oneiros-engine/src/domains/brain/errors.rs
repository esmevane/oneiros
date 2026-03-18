use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum BrainError {
    #[error("Brain not found: {0}")]
    NotFound(String),

    #[error("Brain already exists: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for BrainError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            BrainError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BrainError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            BrainError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
