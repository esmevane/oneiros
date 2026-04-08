use crate::{BlobError, IdParseError, TimestampParseError};
use lorosurgeon::{HydrateError, ReconcileError};

/// Event infrastructure errors.
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IdParse(#[from] IdParseError),

    #[error(transparent)]
    TimestampParse(#[from] TimestampParseError),

    #[error(transparent)]
    Blob(#[from] BlobError),

    #[error(transparent)]
    Loro(#[from] loro::LoroError),

    #[error(transparent)]
    Reconcile(#[from] ReconcileError),

    #[error(transparent)]
    Hydrate(#[from] HydrateError),

    #[error("Import error: {0}")]
    Import(String),

    #[error("Database lock poisoned: {0}")]
    Lock(String),
}
