use crate::PlatformError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum MigrationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform error: {0}")]
    Platform(#[from] PlatformError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Backup failed: {reason}")]
    Backup { reason: String },

    #[error("Migration '{name}' failed: {reason}")]
    Step { name: &'static str, reason: String },
}
