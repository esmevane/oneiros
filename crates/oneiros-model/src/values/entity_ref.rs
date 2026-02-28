use serde::{Deserialize, Serialize};

use super::resource::Resource;
use crate::*;

/// An error that can occur when parsing or encoding a [`RefToken`].
#[derive(Debug, thiserror::Error)]
pub enum RefError {
    #[error("invalid ref encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid ref format: {0}")]
    Format(#[from] postcard::Error),
}

/// A versioned, self-describing reference to any entity in the system.
///
/// Serializes as structural JSON (e.g. `{"V0": {"Agent": "019c-abcd-..."}}`).
/// For opaque string encoding (DB columns, CLI args, query params), use [`RefToken`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ref {
    V0(Resource),
}

impl Ref {
    pub fn agent(id: AgentId) -> Self {
        Self::V0(Resource::Agent(id))
    }

    pub fn actor(id: ActorId) -> Self {
        Self::V0(Resource::Actor(id))
    }

    pub fn brain(id: BrainId) -> Self {
        Self::V0(Resource::Brain(id))
    }

    pub fn cognition(id: CognitionId) -> Self {
        Self::V0(Resource::Cognition(id))
    }

    pub fn connection(id: ConnectionId) -> Self {
        Self::V0(Resource::Connection(id))
    }

    pub fn experience(id: ExperienceId) -> Self {
        Self::V0(Resource::Experience(id))
    }

    pub fn level(name: LevelName) -> Self {
        Self::V0(Resource::Level(name))
    }

    pub fn memory(id: MemoryId) -> Self {
        Self::V0(Resource::Memory(id))
    }

    pub fn nature(name: NatureName) -> Self {
        Self::V0(Resource::Nature(name))
    }

    pub fn persona(name: PersonaName) -> Self {
        Self::V0(Resource::Persona(name))
    }

    pub fn sensation(name: SensationName) -> Self {
        Self::V0(Resource::Sensation(name))
    }

    pub fn storage(key: StorageKey) -> Self {
        Self::V0(Resource::Storage(key))
    }

    pub fn tenant(id: TenantId) -> Self {
        Self::V0(Resource::Tenant(id))
    }

    pub fn texture(name: TextureName) -> Self {
        Self::V0(Resource::Texture(name))
    }

    /// The resource this ref points to.
    pub fn resource(&self) -> &Resource {
        let Self::V0(resource) = self;
        resource
    }

    /// Encode to postcard bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).expect("ref serialization should not fail")
    }

    /// Decode from postcard bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RefError> {
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
