#[derive(thiserror::Error, Debug)]
pub enum InitSystemError {
    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
