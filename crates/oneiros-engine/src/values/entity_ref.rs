use serde::{Deserialize, Serialize};

use super::resource::Resource;
use crate::*;

/// An error that can occur when parsing or encoding a [`RefToken`].
#[derive(Debug, thiserror::Error)]
pub(crate) enum RefError {
    #[error("invalid ref encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid ref format: {0}")]
    Format(#[from] postcard::Error),
}

/// A versioned, self-describing reference to any entity in the system.
///
/// Serializes as structural JSON (e.g. `{"V0": {"Agent": "019c-abcd-..."}}`).
/// For opaque string encoding (DB columns, CLI args, query params), use [`RefToken`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
pub(crate) enum Ref {
    V0(Resource),
}

impl Ref {
    pub(crate) fn agent(id: AgentId) -> Self {
        Self::V0(Resource::Agent(id))
    }

    pub(crate) fn actor(id: ActorId) -> Self {
        Self::V0(Resource::Actor(id))
    }

    pub(crate) fn bookmark(id: BookmarkId) -> Self {
        Self::V0(Resource::Bookmark(id))
    }

    pub(crate) fn brain(id: BrainId) -> Self {
        Self::V0(Resource::Brain(id))
    }

    pub(crate) fn cognition(id: CognitionId) -> Self {
        Self::V0(Resource::Cognition(id))
    }

    pub(crate) fn connection(id: ConnectionId) -> Self {
        Self::V0(Resource::Connection(id))
    }

    pub(crate) fn experience(id: ExperienceId) -> Self {
        Self::V0(Resource::Experience(id))
    }

    pub(crate) fn follow(id: FollowId) -> Self {
        Self::V0(Resource::Follow(id))
    }

    pub(crate) fn level(name: LevelName) -> Self {
        Self::V0(Resource::Level(name))
    }

    pub(crate) fn memory(id: MemoryId) -> Self {
        Self::V0(Resource::Memory(id))
    }

    pub(crate) fn nature(name: NatureName) -> Self {
        Self::V0(Resource::Nature(name))
    }

    pub(crate) fn peer(id: PeerId) -> Self {
        Self::V0(Resource::Peer(id))
    }

    pub(crate) fn persona(name: PersonaName) -> Self {
        Self::V0(Resource::Persona(name))
    }

    pub(crate) fn sensation(name: SensationName) -> Self {
        Self::V0(Resource::Sensation(name))
    }

    pub(crate) fn storage(key: StorageKey) -> Self {
        Self::V0(Resource::Storage(key))
    }

    pub(crate) fn tenant(id: TenantId) -> Self {
        Self::V0(Resource::Tenant(id))
    }

    pub(crate) fn texture(name: TextureName) -> Self {
        Self::V0(Resource::Texture(name))
    }

    pub(crate) fn ticket(id: TicketId) -> Self {
        Self::V0(Resource::Ticket(id))
    }

    pub(crate) fn urge(name: UrgeName) -> Self {
        Self::V0(Resource::Urge(name))
    }

    /// The resource this ref points to.
    pub(crate) fn resource(&self) -> &Resource {
        let Self::V0(resource) = self;
        resource
    }

    /// Encode to postcard bytes.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).expect("ref serialization should not fail")
    }

    /// Decode from postcard bytes.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, RefError> {
        Ok(postcard::from_bytes(bytes)?)
    }
}

impl core::fmt::Display for Ref {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self::V0(resource) = self;
        write!(f, "{resource}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ref_structural_serde_roundtrip() {
        let r = Ref::agent(AgentId::new());
        let json = serde_json::to_value(&r).unwrap();
        // Should be a JSON object, not a string
        assert!(json.is_object());
        let decoded: Ref = serde_json::from_value(json).unwrap();
        assert_eq!(r, decoded);
    }

    #[test]
    fn ref_display_is_resource_form() {
        let id = AgentId::new();
        let r = Ref::agent(id);
        let display = r.to_string();
        assert!(display.starts_with("agent:"), "got: {display}");
    }
}
