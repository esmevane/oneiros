mod agent;
mod brain;
mod cognition;
mod connection;
mod experience;
mod level;
mod lifecycle;
mod memory;
mod nature;
mod persona;
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
pub use experience::*;
pub use level::*;
pub use lifecycle::*;
pub use memory::*;
pub use nature::*;
pub use persona::*;
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
    Experience(ExperienceRequests),
    Level(LevelRequests),
    Lifecycle(LifecycleRequests),
    Memory(MemoryRequests),
    Nature(NatureRequests),
    Persona(PersonaRequests),
    Sensation(SensationRequests),
    Storage(StorageRequests),
    Texture(TextureRequests),
}
