use crate::ContextError;

#[derive(thiserror::Error, Debug)]
pub enum StorageCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
