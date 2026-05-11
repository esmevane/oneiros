use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub(crate) enum SetupError {
    #[error(transparent)]
    System(#[from] crate::SystemError),

    #[error(transparent)]
    Project(#[from] crate::ProjectError),

    #[error(transparent)]
    Seed(#[from] crate::SeedError),

    #[error(transparent)]
    McpConfig(#[from] crate::McpConfigError),

    #[error(transparent)]
    Service(#[from] crate::ServiceError),

    #[error(transparent)]
    HostKey(#[from] crate::HostKeyError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),

    #[error(transparent)]
    Upcast(#[from] crate::UpcastError),
}

impl IntoResponse for SetupError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(self.to_string())),
        )
            .into_response()
    }
}
