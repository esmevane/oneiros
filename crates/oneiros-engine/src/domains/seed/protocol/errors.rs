use axum::response::{IntoResponse, Response};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error(transparent)]
    Level(#[from] LevelError),

    #[error(transparent)]
    Persona(#[from] PersonaError),

    #[error(transparent)]
    Nature(#[from] NatureError),

    #[error(transparent)]
    Sensation(#[from] SensationError),

    #[error(transparent)]
    Urge(#[from] UrgeError),

    #[error(transparent)]
    Texture(#[from] TextureError),

    #[error(transparent)]
    Agent(#[from] AgentError),

    #[error(transparent)]
    Event(#[from] EventError),

    #[error("Required personas (process, scribe) not found. Run `oneiros seed core` first.")]
    MissingPersonas,
}

impl IntoResponse for SeedError {
    fn into_response(self) -> Response {
        match self {
            SeedError::Persona(persona) => persona.into_response(),
            SeedError::Sensation(sensation) => sensation.into_response(),
            SeedError::Nature(nature) => nature.into_response(),
            SeedError::Texture(texture) => texture.into_response(),
            SeedError::Urge(urge) => urge.into_response(),
            SeedError::Level(level) => level.into_response(),
            SeedError::Agent(agent) => agent.into_response(),
            SeedError::Event(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            )
                .into_response(),
            SeedError::MissingPersonas => (
                axum::http::StatusCode::PRECONDITION_FAILED,
                self.to_string(),
            )
                .into_response(),
        }
    }
}
