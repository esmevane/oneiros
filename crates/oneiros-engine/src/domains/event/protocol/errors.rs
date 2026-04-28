use crate::{BlobError, IdParseError, TimestampParseError, UpcastError};

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
    Upcast(#[from] UpcastError),

    #[error("Import error: {0}")]
    Import(String),

    #[error("Database lock poisoned: {0}")]
    Lock(String),
}
