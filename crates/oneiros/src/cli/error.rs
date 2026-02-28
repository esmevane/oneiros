use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Precondition(#[from] ContextError),
    #[error(transparent)]
    Activity(#[from] ActivityError),
    #[error(transparent)]
    Agent(#[from] AgentCommandError),
    #[error(transparent)]
    Cognition(#[from] CognitionCommandError),
    #[error(transparent)]
    Connection(#[from] ConnectionCommandError),
    #[error(transparent)]
    Doctor(#[from] CheckupError),
    #[error(transparent)]
    Dream(#[from] DreamError),
    #[error(transparent)]
    Emerge(#[from] EmergeError),
    #[error(transparent)]
    Experience(#[from] ExperienceCommandError),
    #[error(transparent)]
    Event(#[from] EventCommandError),
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
    Nature(#[from] NatureCommandError),
    #[error(transparent)]
    Persona(#[from] PersonaCommandError),
    #[error(transparent)]
    Recede(#[from] RecedeError),
    #[error(transparent)]
    Reflect(#[from] ReflectError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Seed(#[from] SeedCommandError),
    #[error(transparent)]
    Sense(#[from] SenseError),
    #[error(transparent)]
    Skill(#[from] SkillCommandError),
    #[error(transparent)]
    Sleep(#[from] SleepError),
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
    #[error(transparent)]
    Wake(#[from] WakeError),
}
