use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum LensError {
    #[error(transparent)]
    Parse(#[from] LensParseError),

    #[error(transparent)]
    Validate(#[from] LensValidationError),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for LensError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            LensError::Parse(_) | LensError::Validate(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            LensError::Event(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            LensError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            LensError::Json(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
