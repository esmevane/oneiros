use thiserror::Error;

use crate::Link;

#[derive(Debug, Error)]
pub enum LinkError {
    #[error("failed to serialize link content: {0}")]
    Serialization(postcard::Error),

    #[error("failed to decode link: {0}")]
    Decoding(String),

    #[error(transparent)]
    Narrowing(#[from] LinkNarrowingError),
}

impl From<postcard::Error> for LinkError {
    fn from(error: postcard::Error) -> Self {
        Self::Serialization(error)
    }
}

/// Error when parsing a string as a [`Key`](crate::Key) fails.
///
/// Neither the Id nor the Link parse succeeded.
#[derive(Debug, Clone, Error)]
#[error("could not parse as key: {input}")]
pub struct KeyParseError {
    /// The original input that could not be parsed.
    pub input: String,
}

/// Error when narrowing a [`Link`] into a typed domain link fails.
///
/// The link was valid but its resource label didn't match the expected type.
#[derive(Debug, Clone, Error)]
#[error("expected {expected} link, got {link}")]
pub struct LinkNarrowingError {
    /// The expected resource label (e.g., "agent", "cognition").
    pub expected: &'static str,
    /// The original link that failed to narrow.
    pub link: Link,
}
