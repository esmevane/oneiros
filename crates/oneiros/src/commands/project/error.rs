#[derive(thiserror::Error, Debug)]
pub enum ProjectCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error("IO error: {0}")]
    Io(std::io::Error),

    #[error("No project detected. Run this from within a project directory.")]
    NoProject,
}
