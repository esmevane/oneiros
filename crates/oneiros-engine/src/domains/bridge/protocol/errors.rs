use crate::*;

/// Iroh transport errors — contained within the bridge domain so iroh
/// types don't leak across the API boundary.
#[derive(Debug, thiserror::Error)]
pub enum IrohError {
    #[error("bind failed: {0}")]
    Bind(#[from] iroh::endpoint::BindError),

    #[error("connect failed: {0}")]
    Connect(#[from] iroh::endpoint::ConnectError),

    #[error("connection error: {0}")]
    Connection(#[from] iroh::endpoint::ConnectionError),

    #[error("write error: {0}")]
    Write(#[from] iroh::endpoint::WriteError),

    #[error("read error: {0}")]
    ReadExact(#[from] iroh::endpoint::ReadExactError),

    #[error("stream closed: {0}")]
    Closed(#[from] iroh::endpoint::ClosedStream),
}

/// Errors from bridge operations.
#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    /// An iroh transport error.
    #[error(transparent)]
    Iroh(#[from] IrohError),

    /// The sync protocol encountered a malformed or unexpected message.
    #[error("protocol error: {0}")]
    Protocol(String),

    /// The peer denied the sync request.
    #[error("sync denied: {0}")]
    Denied(String),

    /// An event infrastructure error during sync handling.
    #[error(transparent)]
    Event(#[from] EventError),

    /// Failed to parse an ID from the sync payload.
    #[error(transparent)]
    IdParse(#[from] IdParseError),

    /// Database error during sync handling.
    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    /// A scope hydration error during sync handling.
    #[error(transparent)]
    Scope(#[from] ScopeError),

    /// A scope composition error during sync handling.
    #[error(transparent)]
    Compose(#[from] ComposeError),
}
