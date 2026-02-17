mod actor;
mod agent;
mod brain;
mod cognition;
mod dreaming;
mod experience;
mod introspecting;
mod level;
mod memory;
mod persona;
mod reflecting;
mod responses;
mod sensation;
mod storage;
mod tenant;
mod texture;
mod ticket;

pub use actor::*;
pub use agent::*;
pub use brain::*;
pub use cognition::*;
pub use dreaming::*;
pub use experience::*;
pub use introspecting::*;
pub use level::*;
pub use memory::*;
pub use persona::*;
pub use reflecting::*;
pub use responses::*;
pub use sensation::*;
pub use storage::*;
pub use tenant::*;
pub use texture::*;
pub use ticket::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Cognition(CognitionEvents),
    Dreaming(DreamingEvents),
    Experience(ExperienceEvents),
    Introspecting(IntrospectingEvents),
    Level(LevelEvents),
    Memory(MemoryEvents),
    Persona(PersonaEvents),
    Reflecting(ReflectingEvents),
    Sensation(SensationEvents),
    Storage(StorageEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Requests {
    Actor(ActorRequests),
    Agent(AgentRequests),
    Brain(BrainRequests),
    Cognition(CognitionRequests),
    Dreaming(DreamingRequests),
    Experience(ExperienceRequests),
    Introspecting(IntrospectingRequests),
    Level(LevelRequests),
    Memory(MemoryRequests),
    Persona(PersonaRequests),
    Reflecting(ReflectingRequests),
    Sensation(SensationRequests),
    Storage(StorageRequests),
    Tenant(TenantRequests),
    Texture(TextureRequests),
}
