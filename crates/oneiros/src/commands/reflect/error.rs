use crate::ContextError;

#[derive(thiserror::Error, Debug)]
pub enum ReflectError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}
