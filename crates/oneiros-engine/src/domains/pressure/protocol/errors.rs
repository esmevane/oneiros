use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum PressureError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

impl IntoResponse for PressureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PressureError::Database(_) | PressureError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            PressureError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
