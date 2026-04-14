use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service not initialized — run `oneiros system init` first")]
    NotInitialized,

    #[error("Service manager error: {0}")]
    Manager(String),

    #[error("Health check failed: {0}")]
    HealthCheck(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Server(#[from] ServerError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServiceError::NotInitialized => (StatusCode::PRECONDITION_FAILED, self.to_string()),
            ServiceError::Manager(_) | ServiceError::Server(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ServiceError::HealthCheck(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ServiceError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
