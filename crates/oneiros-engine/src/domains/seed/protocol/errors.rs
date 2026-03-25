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
}

impl IntoResponse for SeedError {
    fn into_response(self) -> Response {
        match self {
            SeedError::Persona(ticket) => ticket.into_response(),
            SeedError::Sensation(sensation) => sensation.into_response(),
            SeedError::Nature(nature) => nature.into_response(),
            SeedError::Texture(texture) => texture.into_response(),
            SeedError::Urge(urge) => urge.into_response(),
            SeedError::Level(level) => level.into_response(),
        }
    }
}
