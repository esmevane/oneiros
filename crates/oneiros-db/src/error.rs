use oneiros_model::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Invalid ID: {0}")]
    Id(#[from] IdParseError),

    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Unable to serialize JSON data to event: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to construct domain type: {0}")]
    Construction(#[from] ConstructionError),

    #[error("Failed to parse ref: {0}")]
    Ref(#[from] RefError),

    #[error("Cannot import unsourced event — call with_source() first")]
    UnsourcedImport,

    #[error("Blob decode failed: {0}")]
    Blob(#[from] BlobError),
}

impl From<AgentConstructionError> for DatabaseError {
    fn from(e: AgentConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<CognitionConstructionError> for DatabaseError {
    fn from(e: CognitionConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<ConnectionConstructionError> for DatabaseError {
    fn from(e: ConnectionConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<MemoryConstructionError> for DatabaseError {
    fn from(e: MemoryConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<ExperienceConstructionError> for DatabaseError {
    fn from(e: ExperienceConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl DatabaseError {
    /// Returns `true` if this error is a SQLite foreign key constraint violation.
    ///
    /// Useful in projection `apply` functions that need to tolerate missing
    /// referential dependencies during import (e.g., a `storage-set` event
    /// arriving before its blob).
    pub fn is_foreign_key_violation(&self) -> bool {
        match self {
            Self::Sqlite(rusqlite::Error::SqliteFailure(err, _)) => {
                // extended_code 787 == SQLITE_CONSTRAINT_FOREIGNKEY
                err.extended_code == 787
            }
            _ => false,
        }
    }
}
