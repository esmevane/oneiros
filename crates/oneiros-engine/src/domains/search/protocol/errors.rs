use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SearchError::Database(_) | SearchError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            SearchError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
