use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::extractors::ActorContextError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unauthorized(#[from] ActorContextError),

    #[error(transparent)]
    Service(#[from] oneiros_service::Error),

    #[error("Unable to access system projects")]
    ProjectExtractionFailure,

    #[error("Unable to summarize project")]
    ProjectSummaryFailure,
}

// Transitive From impls so `?` works in handlers that produce
// service-level errors (NotFound, Conflicts, DatabaseError, etc.)

impl From<oneiros_db::DatabaseError> for Error {
    fn from(err: oneiros_db::DatabaseError) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<oneiros_model::NotFound> for Error {
    fn from(err: oneiros_model::NotFound) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<oneiros_model::Conflicts> for Error {
    fn from(err: oneiros_model::Conflicts) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<oneiros_model::DataIntegrity> for Error {
    fn from(err: oneiros_model::DataIntegrity) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<oneiros_service::PreconditionFailure> for Error {
    fn from(err: oneiros_service::PreconditionFailure) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl From<oneiros_service::BadRequests> for Error {
    fn from(err: oneiros_service::BadRequests) -> Self {
        Error::Service(oneiros_service::Error::from(err))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized(err) => {
                let body = serde_json::json!({ "error": err.to_string() });
                (StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
            }
            Error::ProjectExtractionFailure | Error::ProjectSummaryFailure => {
                let body = serde_json::json!({ "error": self.to_string() });
                (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(body)).into_response()
            }
            Error::Service(err) => {
                let status = match &err {
                    oneiros_service::Error::NotFound(_) => StatusCode::NOT_FOUND,
                    oneiros_service::Error::NotInitialized(_) => StatusCode::PRECONDITION_FAILED,
                    oneiros_service::Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                    oneiros_service::Error::Conflict(_) => StatusCode::CONFLICT,
                    oneiros_service::Error::DataIntegrity(_)
                    | oneiros_service::Error::BlobContent(_)
                    | oneiros_service::Error::Blob(_)
                    | oneiros_service::Error::Database(_)
                    | oneiros_service::Error::Io(_)
                    | oneiros_service::Error::DatabasePoisoned => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let body = serde_json::json!({ "error": err.to_string() });
                (status, axum::Json(body)).into_response()
            }
        }
    }
}
