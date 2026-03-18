use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage entry not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::IoError(e.to_string())
    }
}

impl IntoResponse for StorageError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            StorageError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            StorageError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
