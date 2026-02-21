use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum CognitionConstructionError {
    #[error("invalid cognition id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampConstructionFailure),
}
