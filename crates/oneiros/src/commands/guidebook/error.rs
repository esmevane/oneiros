use crate::ContextError;

#[derive(thiserror::Error, Debug)]
pub enum GuidebookError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}
