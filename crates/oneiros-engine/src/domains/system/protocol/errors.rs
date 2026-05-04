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

    #[error(transparent)]
    Event(#[from] EventError),

    #[error(transparent)]
    HostKey(#[from] HostKeyError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Upcast(#[from] UpcastError),

    #[error(transparent)]
    Compose(#[from] ComposeError),

    #[error(transparent)]
    Client(#[from] ClientError),
}

resource_op_error!(SystemError);

impl IntoResponse for SystemError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SystemError::Tenant(_) | SystemError::Actor(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            SystemError::Database(_)
            | SystemError::Event(_)
            | SystemError::Io(_)
            | SystemError::HostKey(_)
            | SystemError::Upcast(_)
            | SystemError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            SystemError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
