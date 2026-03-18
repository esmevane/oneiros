use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum ActorError {
    #[error("Actor not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for ActorError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ActorError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ActorError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
