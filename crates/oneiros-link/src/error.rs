use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinkError {
    #[error("failed to serialize link content: {0}")]
    Serialization(postcard::Error),

    #[error("failed to decode link: {0}")]
    Decoding(String),

    #[error("failed to encode link: {0}")]
    Encoding(String),
}

impl From<postcard::Error> for LinkError {
    fn from(error: postcard::Error) -> Self {
        Self::Serialization(error)
    }
}
