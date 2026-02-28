use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum ConstructionError {
    #[error(transparent)]
    Agent(#[from] AgentConstructionError),
    #[error(transparent)]
    Cognition(#[from] CognitionConstructionError),
    #[error(transparent)]
    Connection(#[from] ConnectionConstructionError),
    #[error(transparent)]
    Memory(#[from] MemoryConstructionError),
    #[error(transparent)]
    Experience(#[from] ExperienceConstructionError),
}
