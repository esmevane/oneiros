use thiserror::Error;

use crate::{
    AgentCommandError, CheckupError, CognitionCommandError, ContextError, DreamError,
    ExperienceCommandError, GuidebookError, IntrospectError, LevelCommandError, MemoryCommandError,
    PersonaCommandError, ProjectCommandError, ReflectError, SeedCommandError,
    SensationCommandError, SenseError, ServiceCommandError, SkillCommandError, StatusError,
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
    Experience(#[from] ExperienceCommandError),
    #[error(transparent)]
    Sensation(#[from] SensationCommandError),
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
    Sense(#[from] SenseError),
    #[error(transparent)]
    Skill(#[from] SkillCommandError),
    #[error(transparent)]
    Status(#[from] StatusError),
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
