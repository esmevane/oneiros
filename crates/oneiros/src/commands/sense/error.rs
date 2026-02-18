use crate::ContextError;

#[derive(thiserror::Error, Debug)]
pub enum SenseError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Failed to read from stdin: {0}")]
    Stdin(#[from] std::io::Error),
}
