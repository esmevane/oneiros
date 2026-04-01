use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
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
