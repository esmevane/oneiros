use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{ErrorResponse, TenantId, TimestampParseError, resource_op_error};

#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found: {0}")]
    NotFound(TenantId),

    #[error("Invalid ID: {0}")]
    InvalidId(#[from] crate::IdParseError),

    #[error(transparent)]
    Resolve(#[from] crate::ResolveError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    HostDb(#[from] crate::HostDbError),

    #[error(transparent)]
    Event(#[from] crate::EventError),

    #[error(transparent)]
    TimestampParse(#[from] TimestampParseError),

    #[error(transparent)]
    Client(#[from] crate::ClientError),

    #[error(transparent)]
    Compose(#[from] crate::ComposeError),
}

resource_op_error!(TenantError);

impl IntoResponse for TenantError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            TenantError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TenantError::InvalidId(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            TenantError::Resolve(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            TenantError::TimestampParse(_)
            | TenantError::Event(_)
            | TenantError::Database(_)
            | TenantError::HostDb(_)
            | TenantError::Client(_)
            | TenantError::Compose(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
