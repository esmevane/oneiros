use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage key not found: {0}")]
    KeyNotFound(crate::StorageKey),

    #[error("Invalid storage reference")]
    InvalidRef,

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error("Blob missing for hash: {0}")]
    BlobMissing(crate::ContentHash),

    #[error("Blob error: {0}")]
    BlobError(#[from] crate::BlobError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Compose(#[from] crate::ComposeError),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(StorageError);

impl IntoResponse for StorageError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            StorageError::KeyNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            StorageError::InvalidRef => (StatusCode::BAD_REQUEST, self.to_string()),
            StorageError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            StorageError::BlobMissing(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::BlobError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            StorageError::Database(_) | StorageError::Event(_) | StorageError::Compose(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            StorageError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
