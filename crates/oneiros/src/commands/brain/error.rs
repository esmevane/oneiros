#[derive(thiserror::Error, Debug)]
pub enum BrainCommandError {
    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Rewrite error: {0}")]
    Rewrite(#[from] oneiros_service::replay::RewriteError),

    #[error("No project detected. Run this from within a project directory.")]
    NoProject,

    #[error("No brain database found at {0}")]
    NoBrainDb(std::path::PathBuf),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
