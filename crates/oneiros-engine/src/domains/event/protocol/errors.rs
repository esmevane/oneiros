use crate::{BlobError, IdParseError, TimestampParseError};

/// Event store errors.
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

    #[error("Import error: {0}")]
    Import(String),
}
