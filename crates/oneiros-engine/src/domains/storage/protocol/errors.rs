use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage key not found: {0}")]
    KeyNotFound(crate::StorageKey),

    #[error("Blob missing for hash: {0}")]
    BlobMissing(crate::ContentHash),

    #[error("Blob error: {0}")]
    BlobError(#[from] crate::BlobError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for StorageError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            StorageError::KeyNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            StorageError::BlobMissing(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::BlobError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
