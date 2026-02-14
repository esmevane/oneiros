use thiserror::Error;

use crate::{
    AgentCommandError, CheckupError, CognitionCommandError, ContextError, DreamError,
    GuidebookError, IntrospectError, LevelCommandError, MemoryCommandError, PersonaCommandError,
    ProjectCommandError, ReflectError, SeedCommandError, ServiceCommandError, SkillCommandError,
    StorageCommandError, SystemCommandError, TextureCommandError,
};

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Precondition(#[from] ContextError),
    #[error(transparent)]
    Agent(#[from] AgentCommandError),
    #[error(transparent)]
    Cognition(#[from] CognitionCommandError),
    #[error(transparent)]
    Doctor(#[from] CheckupError),
    #[error(transparent)]
    Dream(#[from] DreamError),
    #[error(transparent)]
    Guidebook(#[from] GuidebookError),
    #[error(transparent)]
    Introspect(#[from] IntrospectError),
    #[error(transparent)]
    Level(#[from] LevelCommandError),
    #[error(transparent)]
    Memory(#[from] MemoryCommandError),
    #[error(transparent)]
    Persona(#[from] PersonaCommandError),
    #[error(transparent)]
    Reflect(#[from] ReflectError),
    #[error(transparent)]
    Seed(#[from] SeedCommandError),
    #[error(transparent)]
    Skill(#[from] SkillCommandError),
    #[error(transparent)]
    Storage(#[from] StorageCommandError),
    #[error(transparent)]
    Project(#[from] ProjectCommandError),
    #[error(transparent)]
    Service(#[from] ServiceCommandError),
    #[error(transparent)]
    System(#[from] SystemCommandError),
    #[error(transparent)]
    Texture(#[from] TextureCommandError),
}
