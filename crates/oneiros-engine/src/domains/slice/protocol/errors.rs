use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum SliceError {
    #[error("slice not found: {0}")]
    NotFound(SliceName),
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error(transparent)]
    Compose(#[from] ComposeError),
    #[error(transparent)]
    Lens(#[from] LensError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

resource_op_error!(SliceError);

impl IntoResponse for SliceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SliceError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            SliceError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            SliceError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            SliceError::Lens(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            SliceError::Json(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
