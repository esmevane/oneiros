use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not initialized: {0}")]
    NotInitialized(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Internal error: lock poisoned")]
    LockPoisoned,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::NotInitialized(_) => StatusCode::PRECONDITION_FAILED,
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::LockPoisoned => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}
