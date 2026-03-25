use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error(transparent)]
    Tenant(#[from] TenantError),

    #[error(transparent)]
    Actor(#[from] ActorError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),
}

impl IntoResponse for SystemError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SystemError::Tenant(_) | SystemError::Actor(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            SystemError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
