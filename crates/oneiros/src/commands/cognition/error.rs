use crate::{ContextError, PrefixError};

#[derive(thiserror::Error, Debug)]
pub enum CognitionCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(transparent)]
    PrefixResolve(#[from] PrefixError),
}
