use crate::*;

/// Iroh transport errors — contained within the bridge domain so iroh
/// types don't leak across the API boundary.
#[derive(Debug, thiserror::Error)]
pub(crate) enum IrohError {
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

/// Which bridge operation a protocol error pertains to.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum BridgeOp {
    Diff,
    Resolve,
    FetchEvents,
    ListBookmarks,
    PullBookmark,
    PushBookmark,
}

impl core::fmt::Display for BridgeOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Diff => f.write_str("bridge-diff"),
            Self::Resolve => f.write_str("bridge-resolve"),
            Self::FetchEvents => f.write_str("bridge-fetch-events"),
            Self::ListBookmarks => f.write_str("bridge-list-bookmarks"),
            Self::PullBookmark => f.write_str("bridge-pull-bookmark"),
            Self::PushBookmark => f.write_str("bridge-push-bookmark"),
        }
    }
}

/// A violation of the sync wire protocol — malformed or unexpected
/// messages, oversized responses, decode failures.
#[derive(Debug, thiserror::Error)]
pub(crate) enum BridgeProtocolError {
    #[error("chronicle root node not found in store")]
    ChronicleRootMissing,

    #[error("unexpected response to {0} request")]
    UnexpectedResponse(BridgeOp),

    #[error("response too large: {0} bytes")]
    ResponseTooLarge(usize),

    #[error("malformed response: {0}")]
    Decode(#[from] serde_json::Error),
}

/// An opaque human-readable message received from a peer. We don't
/// interpret the contents — the peer chose this text and we preserve
/// it verbatim for surfacing in logs and error responses.
#[derive(Debug, Clone)]
pub(crate) struct OpaquePeer(String);

impl From<String> for OpaquePeer {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for OpaquePeer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Why a sync request was denied. Internal-typed; renders to the
/// human-readable `reason` carried by the wire-level `BridgeDenied`.
#[derive(Debug, thiserror::Error)]
pub(crate) enum DenyReason {
    #[error("ticket not found")]
    TicketNotFound,

    #[error("link target does not match ticket target")]
    TargetMismatch,

    #[error("ticket does not grant the required permission")]
    InsufficientPermissions,

    #[error(transparent)]
    Invalid(#[from] TicketInvalid),

    /// A denial received over the wire from a peer.
    #[error("{0}")]
    Remote(OpaquePeer),
}

/// Errors from bridge operations.
#[derive(Debug, thiserror::Error)]
pub(crate) enum BridgeError {
    /// An iroh transport error.
    #[error(transparent)]
    Iroh(#[from] IrohError),

    /// The sync protocol encountered a malformed or unexpected message.
    #[error("protocol error: {0}")]
    Protocol(#[from] BridgeProtocolError),

    /// The peer denied the sync request.
    #[error("sync denied: {0}")]
    Denied(#[from] DenyReason),

    /// An event infrastructure error during sync handling.
    #[error(transparent)]
    Event(#[from] EventError),

    /// Failed to parse an ID from the sync payload.
    #[error(transparent)]
    IdParse(#[from] IdParseError),

    /// Database error during sync handling.
    #[error(transparent)]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    HostDb(#[from] HostDbError),

    #[error(transparent)]
    EventsDb(#[from] EventsDbError),

    /// A scope hydration error during sync handling.
    #[error(transparent)]
    Scope(#[from] ScopeError),

    /// A scope composition error during sync handling.
    #[error(transparent)]
    Compose(#[from] ComposeError),
}

impl From<TicketInvalid> for BridgeError {
    fn from(value: TicketInvalid) -> Self {
        BridgeError::Denied(DenyReason::Invalid(value))
    }
}
