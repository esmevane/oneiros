use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use oneiros_model::{
    AgentName, BrainId, BrainName, ContentHash, LevelName, LinkError, NatureName, PersonaName,
    SensationName, StorageKey, TextureName,
};

use crate::extractors::ActorContextError;
use crate::handlers::brain::CreateBrainError;

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
}

#[derive(Debug, thiserror::Error)]
pub enum NotFound {
    #[error("Agent not found: {0}")]
    Agent(AgentName),
    #[error("Brain not found: {0}")]
    Brain(BrainId),
    #[error("Cognition not found: {0}")]
    Cognition(String),
    #[error("Connection not found: {0}")]
    Connection(String),
    #[error("Nature not found: {0}")]
    Nature(NatureName),
    #[error("Experience not found: {0}")]
    Experience(String),
    #[error("Sensation not found: {0}")]
    Sensation(SensationName),
    #[error("Level not found: {0}")]
    Level(LevelName),
    #[error("Memory not found: {0}")]
    Memory(String),
    #[error("Persona not found: {0}")]
    Persona(PersonaName),
    #[error("Storage entry not found: {0}")]
    Storage(StorageKey),
    #[error("Texture not found: {0}")]
    Texture(TextureName),
}

#[derive(Debug, thiserror::Error)]
pub enum Conflicts {
    #[error("Agent already exists: {0}")]
    Agent(AgentName),
    #[error("Brain already exists: {0}")]
    Brain(BrainName),
}

#[derive(Debug, thiserror::Error)]
pub enum DataIntegrity {
    #[error("Blob data missing for content hash: {0}")]
    BlobMissing(ContentHash),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unauthorized(#[from] ActorContextError),

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

    #[error("Failed to compute link: {0}")]
    Link(#[from] LinkError),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to acquire database lock")]
    DatabasePoisoned,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::NotInitialized(_) => StatusCode::PRECONDITION_FAILED,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::DataIntegrity(_)
            | Error::Link(_)
            | Error::Database(_)
            | Error::Io(_)
            | Error::DatabasePoisoned => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}
