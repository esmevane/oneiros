use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum UpcastError {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Part of our versioning system, but we haven't versioned yet"
        )
    )]
    #[error("discontinuity {from} -> {to}: {reason}")]
    Discontinuity {
        from: &'static str,
        to: &'static str,
        reason: &'static str,
    },
}

impl From<std::convert::Infallible> for UpcastError {
    fn from(x: std::convert::Infallible) -> Self {
        match x {}
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Agent(#[from] AgentError),
    #[error(transparent)]
    Actor(#[from] ActorError),
    #[error(transparent)]
    Bookmark(#[from] BookmarkError),
    #[error("configuration: {0}")]
    Config(String),
    #[error(transparent)]
    Cognition(#[from] CognitionError),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error(transparent)]
    Doctor(#[from] DoctorError),
    #[error(transparent)]
    Event(#[from] EventError),
    #[error(transparent)]
    Experience(#[from] ExperienceError),
    #[error(transparent)]
    Follow(#[from] FollowError),
    #[error(transparent)]
    Level(#[from] LevelError),
    #[error(transparent)]
    Continuity(#[from] ContinuityError),
    #[error(transparent)]
    McpConfig(#[from] McpConfigError),
    #[error(transparent)]
    Memory(#[from] MemoryError),
    #[error(transparent)]
    Nature(#[from] NatureError),
    #[error(transparent)]
    Peer(#[from] PeerError),
    #[error(transparent)]
    Persona(#[from] PersonaError),
    #[error(transparent)]
    Pressure(#[from] PressureError),
    #[error(transparent)]
    Project(#[from] ProjectError),
    #[error(transparent)]
    Scope(#[from] ScopeError),
    #[error(transparent)]
    Compose(#[from] ComposeError),
    #[error(transparent)]
    Seed(#[from] SeedError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Sensation(#[from] SensationError),
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    Host(#[from] HostError),
    #[error(transparent)]
    Tenant(#[from] TenantError),
    #[error(transparent)]
    Texture(#[from] TextureError),
    #[error(transparent)]
    Ticket(#[from] TicketError),
    #[error(transparent)]
    Trail(#[from] TrailError),
    #[error(transparent)]
    Urge(#[from] UrgeError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discontinuity_renders_from_to_and_reason() {
        let error = UpcastError::Discontinuity {
            from: "FooV1",
            to: "FooV2",
            reason: "field `kind` removed without migration path",
        };

        let rendered = error.to_string();
        assert!(rendered.contains("FooV1"));
        assert!(rendered.contains("FooV2"));
        assert!(rendered.contains("field `kind` removed without migration path"));
    }
}
