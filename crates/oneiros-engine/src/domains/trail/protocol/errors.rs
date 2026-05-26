use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub(crate) enum TrailError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Compose(#[from] crate::ComposeError),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Ref(#[from] crate::RefError),
}

impl IntoResponse for TrailError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            TrailError::Database(_) | TrailError::Event(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(self.to_string()),
            ),
            TrailError::Compose(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(self.to_string()),
            ),
            TrailError::Client(_) | TrailError::Json(_) => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse::new(self.to_string()),
            ),
            TrailError::Ref(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::new(self.to_string()).with_code("invalid_ref"),
            ),
        };
        (status, Json(body)).into_response()
    }
}

resource_op_error!(TrailError);
