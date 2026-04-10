use serde::{Deserialize, Serialize};

use crate::*;

/// All addressable resource types in the system.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
pub enum Resource {
    Agent(AgentId),
    Actor(ActorId),
    Bookmark(BookmarkId),
    Brain(BrainId),
    Cognition(CognitionId),
    Connection(ConnectionId),
    Experience(ExperienceId),
    Follow(FollowId),
    Level(LevelName),
    Memory(MemoryId),
    Nature(NatureName),
    Peer(PeerId),
    Persona(PersonaName),
    Sensation(SensationName),
    Storage(StorageKey),
    Tenant(TenantId),
    Texture(TextureName),
    Ticket(TicketId),
    Urge(UrgeName),
}

impl Resource {
    /// The underlying UUID for ID-keyed resources, or `None` for name-keyed ones.
    pub fn id(&self) -> Option<Id> {
        match self {
            Self::Agent(id) => Some(id.0),
            Self::Actor(id) => Some(id.0),
            Self::Bookmark(id) => Some(id.0),
            Self::Brain(id) => Some(id.0),
            Self::Cognition(id) => Some(id.0),
            Self::Connection(id) => Some(id.0),
            Self::Experience(id) => Some(id.0),
            Self::Follow(id) => Some(id.0),
            Self::Memory(id) => Some(id.0),
            Self::Peer(id) => Some(id.0),
            Self::Tenant(id) => Some(id.0),
            Self::Ticket(id) => Some(id.0),
            Self::Level(_)
            | Self::Nature(_)
            | Self::Persona(_)
            | Self::Sensation(_)
            | Self::Storage(_)
            | Self::Texture(_)
            | Self::Urge(_) => None,
        }
    }

    /// A human-readable label for this resource type.
    pub fn label(&self) -> &str {
        match self {
            Self::Agent(_) => "agent",
            Self::Actor(_) => "actor",
            Self::Bookmark(_) => "bookmark",
            Self::Brain(_) => "brain",
            Self::Cognition(_) => "cognition",
            Self::Connection(_) => "connection",
            Self::Experience(_) => "experience",
            Self::Follow(_) => "follow",
            Self::Level(_) => "level",
            Self::Memory(_) => "memory",
            Self::Nature(_) => "nature",
            Self::Peer(_) => "peer",
            Self::Persona(_) => "persona",
            Self::Sensation(_) => "sensation",
            Self::Storage(_) => "storage",
            Self::Tenant(_) => "tenant",
            Self::Texture(_) => "texture",
            Self::Ticket(_) => "ticket",
            Self::Urge(_) => "urge",
        }
    }
}

impl core::fmt::Display for Resource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Agent(id) => write!(f, "agent:{id}"),
            Self::Actor(id) => write!(f, "actor:{id}"),
            Self::Bookmark(id) => write!(f, "bookmark:{id}"),
            Self::Brain(id) => write!(f, "brain:{id}"),
            Self::Cognition(id) => write!(f, "cognition:{id}"),
            Self::Connection(id) => write!(f, "connection:{id}"),
            Self::Experience(id) => write!(f, "experience:{id}"),
            Self::Follow(id) => write!(f, "follow:{id}"),
            Self::Level(name) => write!(f, "level:{name}"),
            Self::Memory(id) => write!(f, "memory:{id}"),
            Self::Nature(name) => write!(f, "nature:{name}"),
            Self::Peer(id) => write!(f, "peer:{id}"),
            Self::Persona(name) => write!(f, "persona:{name}"),
            Self::Sensation(name) => write!(f, "sensation:{name}"),
            Self::Storage(key) => write!(f, "storage:{key}"),
            Self::Tenant(id) => write!(f, "tenant:{id}"),
            Self::Texture(name) => write!(f, "texture:{name}"),
            Self::Ticket(id) => write!(f, "ticket:{id}"),
            Self::Urge(name) => write!(f, "urge:{name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_label() {
        assert_eq!(Resource::Agent(AgentId::new()).label(), "agent");
        assert_eq!(Resource::Cognition(CognitionId::new()).label(), "cognition");
        assert_eq!(Resource::Texture(TextureName::new("x")).label(), "texture");
        assert_eq!(Resource::Storage(StorageKey::new("k")).label(), "storage");
    }

    #[test]
    fn resource_display() {
        let id = AgentId::new();
        let resource = Resource::Agent(id);
        let display = resource.to_string();
        assert!(display.starts_with("agent:"));
    }
}
