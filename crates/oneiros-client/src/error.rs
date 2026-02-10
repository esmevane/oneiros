use std::sync::Arc;

#[derive(Debug, Clone, thiserror::Error)]
#[error("{0}")]
pub struct ConnectionError(pub Arc<hyper::Error>);

impl From<hyper::Error> for ConnectionError {
    fn from(error: hyper::Error) -> Self {
        Self(Arc::new(error))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RequestError {
    #[error("{0}")]
    Hyper(Arc<hyper::Error>),
    #[error("{0}")]
    Http(Arc<hyper::http::Error>),
}

impl From<hyper::Error> for RequestError {
    fn from(error: hyper::Error) -> Self {
        Self::Hyper(Arc::new(error))
    }
}

impl From<hyper::http::Error> for RequestError {
    fn from(error: hyper::http::Error) -> Self {
        Self::Http(Arc::new(error))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("service returned {status}: {body}")]
pub struct ServiceResponseError {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Connection(#[from] ConnectionError),

    #[error(transparent)]
    Request(#[from] RequestError),

    #[error(transparent)]
    ServiceResponse(#[from] ServiceResponseError),

    #[error("Failed to deserialize response: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
