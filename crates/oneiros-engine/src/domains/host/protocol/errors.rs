use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum HostError {
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

    #[error(transparent)]
    HostDb(#[from] HostDbError),

    #[error(transparent)]
    Server(#[from] ServerError),

    #[error("Service manager error: {0}")]
    Manager(String),

    #[error("Unexpected service response: {0}")]
    UnexpectedResponse(String),
}

resource_op_error!(HostError);

impl IntoResponse for HostError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            HostError::Tenant(_) | HostError::Actor(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            HostError::Database(_)
            | HostError::Event(_)
            | HostError::Io(_)
            | HostError::HostKey(_)
            | HostError::Upcast(_)
            | HostError::Compose(_)
            | HostError::HostDb(_)
            | HostError::Server(_)
            | HostError::Manager(_)
            | HostError::UnexpectedResponse(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            HostError::Client(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
