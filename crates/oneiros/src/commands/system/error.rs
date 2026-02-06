#[derive(thiserror::Error, Debug)]
pub enum SystemCommandError {
    #[error("Error during initialization: {0}")]
    Init(#[from] InitError),
}

#[derive(thiserror::Error, Debug)]
pub enum InitError {
    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Prompt error: {0}")]
    Prompt(#[from] InitPromptError),
}

#[derive(thiserror::Error, Debug)]
pub enum InitPromptError {
    #[error("Error gathering initialization prompt: {0}")]
    Prompt(#[from] inquire::InquireError),
    #[error("Could not detect name. Use --name or run interactively.")]
    FailedToDetect,
}
