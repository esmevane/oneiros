use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    #[error(transparent)]
    System(#[from] SystemError),

    #[error(transparent)]
    Project(#[from] ProjectError),

    #[error(transparent)]
    Seed(#[from] SeedError),

    #[error(transparent)]
    McpConfig(#[from] McpConfigError),

    #[error(transparent)]
    Service(#[from] ServiceError),
}

impl IntoResponse for SetupError {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        )
            .into_response()
    }
}
