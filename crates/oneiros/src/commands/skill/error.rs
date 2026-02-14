use crate::ContextError;

#[derive(thiserror::Error, Debug)]
pub enum SkillCommandError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(
        "No project detected. Use --project from a project directory, or omit it to install globally."
    )]
    NoProject,

    #[error("Could not determine home directory.")]
    NoHomeDir,
}
