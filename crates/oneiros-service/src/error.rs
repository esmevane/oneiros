use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use oneiros_model::*;

use crate::extractors::ActorContextError;
use crate::system_service::CreateBrainError;

#[derive(Debug, thiserror::Error)]
pub enum PreconditionFailure {
    #[error("No tenant found. Run 'oneiros system init' first.")]
    NoTenant,
    #[error("No actor found. Run 'oneiros system init' first.")]
    NoActor,
}

#[derive(Debug, thiserror::Error)]
pub enum BadRequests {
    #[error("Create brain request invalid: {0}")]
    Brain(#[from] CreateBrainError),
    #[error("Invalid storage reference: {0}")]
    StorageRef(#[from] oneiros_model::StorageRefError),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unauthorized(#[from] ActorContextError),

    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Precondition failure: {0}")]
    NotInitialized(#[from] PreconditionFailure),

    #[error(transparent)]
    BadRequest(#[from] BadRequests),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),

    #[error("Data integrity: {0}")]
    DataIntegrity(#[from] DataIntegrity),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to acquire database lock")]
    DatabasePoisoned,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::NotInitialized(_) => StatusCode::PRECONDITION_FAILED,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::DataIntegrity(_)
            | Error::Database(_)
            | Error::Io(_)
            | Error::DatabasePoisoned => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}
