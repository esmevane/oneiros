use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum DoctorError {
    #[error("Database error: {0}")]
    Database(#[from] crate::EventError),
}

impl IntoResponse for DoctorError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            DoctorError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
