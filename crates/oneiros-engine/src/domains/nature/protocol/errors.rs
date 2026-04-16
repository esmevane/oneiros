use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum NatureError {
    #[error("Nature not found: {0}")]
    NotFound(NatureName),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

resource_op_error!(NatureError);

impl IntoResponse for NatureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            NatureError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            NatureError::Database(_) | NatureError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            NatureError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
