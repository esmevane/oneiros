use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum PersonaError {
    #[error("Persona not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for PersonaError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PersonaError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            PersonaError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
