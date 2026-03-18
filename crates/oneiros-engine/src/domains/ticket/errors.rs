use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("Ticket not found: {0}")]
    NotFound(String),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Database error: {0}")]
    Database(#[from] crate::store::StoreError),
}

impl IntoResponse for TicketError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TicketError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TicketError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            TicketError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
