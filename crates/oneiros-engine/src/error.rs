use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Context(String),

    #[error(transparent)]
    Agent(#[from] AgentError),
    #[error(transparent)]
    Actor(#[from] ActorError),
    #[error(transparent)]
    Bookmark(#[from] BookmarkError),
    #[error(transparent)]
    Brain(#[from] BrainError),
    #[error(transparent)]
    Cognition(#[from] CognitionError),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error(transparent)]
    Doctor(#[from] DoctorError),
    #[error(transparent)]
    Event(#[from] EventError),
    #[error(transparent)]
    Experience(#[from] ExperienceError),
    #[error(transparent)]
    Level(#[from] LevelError),
    #[error(transparent)]
    Continuity(#[from] ContinuityError),
    #[error(transparent)]
    McpConfig(#[from] McpConfigError),
    #[error(transparent)]
    Memory(#[from] MemoryError),
    #[error(transparent)]
    Nature(#[from] NatureError),
    #[error(transparent)]
    Persona(#[from] PersonaError),
    #[error(transparent)]
    Pressure(#[from] PressureError),
    #[error(transparent)]
    Project(#[from] ProjectError),
    #[error(transparent)]
    Seed(#[from] SeedError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Sensation(#[from] SensationError),
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    System(#[from] SystemError),
    #[error(transparent)]
    Tenant(#[from] TenantError),
    #[error(transparent)]
    Texture(#[from] TextureError),
    #[error(transparent)]
    Ticket(#[from] TicketError),
    #[error(transparent)]
    Urge(#[from] UrgeError),
}
