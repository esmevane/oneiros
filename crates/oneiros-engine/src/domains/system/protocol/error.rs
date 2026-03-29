use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error(transparent)]
    Tenant(#[from] TenantError),

    #[error(transparent)]
    Actor(#[from] ActorError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),
}
