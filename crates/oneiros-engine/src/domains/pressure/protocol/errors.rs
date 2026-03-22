use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum PressureError {
    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

impl IntoResponse for PressureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PressureError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            PressureError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
