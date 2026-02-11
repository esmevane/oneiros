#[derive(thiserror::Error, Debug)]
pub enum ContextError {
    #[error("No system context available.")]
    NoContext,
    #[error("Unable to parse project name")]
    NoProject,
    #[error("Malformed or missing token file: {0}")]
    MalformedTokenFile(#[from] std::io::Error),
    #[error("Project directory not available")]
    NoProjectDir,
}
