#[derive(thiserror::Error, Debug)]
pub enum ServiceCommandError {
    #[error("Service error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("System not initialized. Run `oneiros system init` first.")]
    NotInitialized,
}
