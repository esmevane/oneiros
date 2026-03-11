use oneiros_model::*;

#[derive(Debug, thiserror::Error)]
pub enum CreateBrainError {
    #[error("Malformed input: {0}")]
    MalformedId(#[from] IdParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum PreconditionFailure {
    #[error("No tenant found. Run 'oneiros system init' first.")]
    NoTenant,
    #[error("No actor found. Run 'oneiros system init' first.")]
    NoActor,
}

#[derive(Debug, thiserror::Error)]
pub enum BadRequests {
    #[error("Create brain request invalid: {0}")]
    Brain(#[from] CreateBrainError),
    #[error("Invalid storage reference: {0}")]
    StorageRef(#[from] oneiros_model::StorageRefError),
    #[error("Request not handled by this service: {0}")]
    NotHandled(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Precondition failure: {0}")]
    NotInitialized(#[from] PreconditionFailure),

    #[error(transparent)]
    BadRequest(#[from] BadRequests),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),

    #[error("Data integrity: {0}")]
    DataIntegrity(#[from] DataIntegrity),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    BlobContent(#[from] BlobContentError),

    #[error(transparent)]
    Blob(#[from] BlobError),

    #[error("Failed to acquire database lock")]
    DatabasePoisoned,

    #[error("Brain context required for this operation")]
    NoBrainContext,

    #[error("Malformed token: {0}")]
    MalformedToken(#[from] TokenError),

    #[error("Invalid or expired ticket")]
    InvalidOrExpiredTicket,
}
