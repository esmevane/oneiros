use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::{ErrorResponse, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),
}

resource_op_error!(SearchError);

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SearchError::Database(_) | SearchError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            SearchError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
