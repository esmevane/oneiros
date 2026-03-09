mod agent;
mod brain;
mod cognition;
mod connection;
mod event_ops;
mod experience;
mod failures;
mod level;
mod lifecycle;
mod memory;
mod nature;
mod persona;
mod search_ops;
mod sensation;
mod serde_stability_tests;
mod storage;
mod tenant;
mod texture;
mod ticket;

use serde::{Deserialize, Serialize};

pub use agent::*;
pub use brain::*;
pub use cognition::*;
pub use connection::*;
pub use event_ops::*;
pub use experience::*;
pub use failures::*;
pub use level::*;
pub use lifecycle::*;
pub use memory::*;
pub use nature::*;
pub use persona::*;
pub use search_ops::*;
pub use sensation::*;
pub use storage::*;
pub use tenant::*;
pub use texture::*;
pub use ticket::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Cognition(CognitionEvents),
    Connection(ConnectionEvents),
    Dreaming(DreamingEvents),
    Experience(ExperienceEvents),
    Introspecting(IntrospectingEvents),
    Level(LevelEvents),
    Lifecycle(LifecycleEvents),
    Memory(MemoryEvents),
    Nature(NatureEvents),
    Persona(PersonaEvents),
    Reflecting(ReflectingEvents),
    Sensation(SensationEvents),
    Sense(SenseEvents),
    Storage(StorageEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
}

/// Super-enum over all request types. Serde untagged — each inner enum
/// carries its own `{type, data}` tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Requests {
    Agent(AgentRequests),
    Brain(BrainRequests),
    Cognition(CognitionRequests),
    Connection(ConnectionRequests),
    Dreaming(DreamingRequests),
    Event(EventRequests),
    Experience(ExperienceRequests),
    Introspecting(IntrospectingRequests),
    Level(LevelRequests),
    Lifecycle(LifecycleRequests),
    Memory(MemoryRequests),
    Nature(NatureRequests),
    Persona(PersonaRequests),
    Reflecting(ReflectingRequests),
    Search(SearchRequests),
    Sensation(SensationRequests),
    Sense(SenseRequests),
    Storage(StorageRequests),
    Texture(TextureRequests),
}

/// Super-enum over all response types. Serde untagged — each inner enum
/// carries its own `{type, data}` tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Responses {
    Agent(AgentResponses),
    Brain(BrainResponses),
    Cognition(CognitionResponses),
    Connection(ConnectionResponses),
    Dreaming(DreamingResponses),
    Event(EventResponses),
    Experience(ExperienceResponses),
    Introspecting(IntrospectingResponses),
    Level(LevelResponses),
    Lifecycle(LifecycleResponses),
    Memory(MemoryResponses),
    Nature(NatureResponses),
    Persona(PersonaResponses),
    Reflecting(ReflectingResponses),
    Search(SearchResponses),
    Sensation(SensationResponses),
    Sense(SenseResponses),
    Storage(StorageResponses),
    Texture(TextureResponses),
}
