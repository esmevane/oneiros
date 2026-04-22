use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SensationError {
    #[error("Sensation not found: {0}")]
    NotFound(SensationName),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

resource_op_error!(SensationError);

impl IntoResponse for SensationError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SensationError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            SensationError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            SensationError::Database(_) | SensationError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            SensationError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
