use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("Texture not found: {0}")]
    NotFound(TextureName),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Event(#[from] EventError),
}

resource_op_error!(TextureError);

impl IntoResponse for TextureError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TextureError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TextureError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            TextureError::Database(_) | TextureError::Event(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            TextureError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
