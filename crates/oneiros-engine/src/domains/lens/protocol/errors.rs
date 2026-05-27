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

    #[error(transparent)]
    BookmarkDb(#[from] crate::BookmarkDbError),

    #[error(transparent)]
    Compile(#[from] CompileError),

    #[error(transparent)]
    Execute(#[from] ExecuteError),
}

impl IntoResponse for LensError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            LensError::Parse(_) | LensError::Validate(_) | LensError::Compile(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            LensError::Execute(_) | LensError::Event(_) | LensError::BookmarkDb(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            LensError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            LensError::Json(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
