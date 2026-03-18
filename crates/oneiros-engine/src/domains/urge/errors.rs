use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum UrgeError {
    #[error("Urge not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for UrgeError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            UrgeError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            UrgeError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
