use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum DoctorError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),
}

impl IntoResponse for DoctorError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            DoctorError::Database(_) | DoctorError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
