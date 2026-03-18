use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("Texture not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for TextureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TextureError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TextureError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
