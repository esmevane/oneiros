use thiserror::Error;

use crate::{
    CheckupError, ContextError, LevelCommandError, PersonaCommandError, ProjectCommandError,
    ServiceCommandError, SystemCommandError, TextureCommandError,
};

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Precondition(#[from] ContextError),
    #[error(transparent)]
    Doctor(#[from] CheckupError),
    #[error(transparent)]
    Level(#[from] LevelCommandError),
    #[error(transparent)]
    Persona(#[from] PersonaCommandError),
    #[error(transparent)]
    Project(#[from] ProjectCommandError),
    #[error(transparent)]
    Service(#[from] ServiceCommandError),
    #[error(transparent)]
    System(#[from] SystemCommandError),
    #[error(transparent)]
    Texture(#[from] TextureCommandError),
}
