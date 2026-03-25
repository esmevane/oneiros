use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum PersonaError {
    #[error("Persona not found: {0}")]
    NotFound(PersonaName),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

impl IntoResponse for PersonaError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            PersonaError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            PersonaError::Database(_) | PersonaError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            PersonaError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
