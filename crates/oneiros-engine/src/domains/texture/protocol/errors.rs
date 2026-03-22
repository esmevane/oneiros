use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("Texture not found: {0}")]
    NotFound(TextureName),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] EventError),
}

impl IntoResponse for TextureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TextureError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TextureError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            TextureError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
