use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum UrgeError {
    #[error("Urge not found: {0}")]
    NotFound(UrgeName),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

resource_op_error!(UrgeError);

impl IntoResponse for UrgeError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            UrgeError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            UrgeError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            UrgeError::Database(_) | UrgeError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            UrgeError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
