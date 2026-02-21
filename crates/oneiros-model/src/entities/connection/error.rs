use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionConstructionError {
    #[error("invalid connection id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid from_link: {0}")]
    InvalidFromLink(oneiros_link::LinkError),
    #[error("invalid to_link: {0}")]
    InvalidToLink(oneiros_link::LinkError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampConstructionFailure),
}
