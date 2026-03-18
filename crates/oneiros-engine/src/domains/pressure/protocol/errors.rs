use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum PressureError {
    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for PressureError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": self.to_string() })),
        )
            .into_response()
    }
}
