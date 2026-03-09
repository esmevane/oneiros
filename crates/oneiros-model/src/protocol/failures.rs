use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum NotFound {
    #[error("Actor not found: {0}")]
    Actor(ActorName),
    #[error("Agent not found: {0}")]
    Agent(AgentName),
    #[error("Brain not found: {0}")]
    Brain(BrainId),
    #[error("Brain not found: {0}")]
    BrainByName(BrainName),
    #[error("Cognition not found: {0}")]
    Cognition(CognitionId),
    #[error("Connection not found: {0}")]
    Connection(ConnectionId),
    #[error("Nature not found: {0}")]
    Nature(NatureName),
    #[error("Experience not found: {0}")]
    Experience(ExperienceId),
    #[error("Sensation not found: {0}")]
    Sensation(SensationName),
    #[error("Level not found: {0}")]
    Level(LevelName),
    #[error("Memory not found: {0}")]
    Memory(MemoryId),
    #[error("Persona not found: {0}")]
    Persona(PersonaName),
    #[error("Storage entry not found: {0}")]
    Storage(StorageKey),
    #[error("Texture not found: {0}")]
    Texture(TextureName),
    #[error("Event not found: {0}")]
    Event(EventId),
    #[error("Tenant not found: {0}")]
    Tenant(TenantName),
}

#[derive(Debug, thiserror::Error)]
pub enum Conflicts {
    #[error("Agent already exists: {0}")]
    Agent(AgentName),
    #[error("Brain already exists: {0}")]
    Brain(BrainName),
}

#[derive(Debug, thiserror::Error)]
pub enum DataIntegrity {
    #[error("Blob data missing for content hash: {0}")]
    BlobMissing(ContentHash),
}
