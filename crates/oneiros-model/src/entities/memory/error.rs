use thiserror::Error;

use crate::IdParseError;

#[derive(Debug, Error)]
pub enum MemoryConstructionError {
    #[error("invalid memory id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(chrono::ParseError),
}
