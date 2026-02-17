use thiserror::Error;

use crate::IdParseError;

#[derive(Debug, Error)]
pub enum AgentConstructionError {
    #[error("invalid agent id: {0}")]
    InvalidId(#[from] IdParseError),
}
