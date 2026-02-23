use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ExperienceConstructionError {
    #[error("invalid experience id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(#[from] KeyParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampConstructionFailure),
}
