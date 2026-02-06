use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Unable to serialize JSON data to event: {0}")]
    Serialization(#[from] serde_json::Error),
}
