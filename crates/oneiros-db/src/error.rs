use oneiros_model::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Unable to serialize JSON data to event: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to construct domain type: {0}")]
    Construction(#[from] ConstructionError),
}

impl From<AgentConstructionError> for DatabaseError {
    fn from(e: AgentConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<CognitionConstructionError> for DatabaseError {
    fn from(e: CognitionConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<MemoryConstructionError> for DatabaseError {
    fn from(e: MemoryConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<ExperienceConstructionError> for DatabaseError {
    fn from(e: ExperienceConstructionError) -> Self {
        Self::Construction(e.into())
    }
}

impl From<LinkConstructionError> for DatabaseError {
    fn from(e: LinkConstructionError) -> Self {
        Self::Construction(e.into())
    }
}
