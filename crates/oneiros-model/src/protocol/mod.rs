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

impl From<AgentRequests> for Requests {
    fn from(r: AgentRequests) -> Self {
        Self::Agent(r)
    }
}
impl From<BrainRequests> for Requests {
    fn from(r: BrainRequests) -> Self {
        Self::Brain(r)
    }
}
impl From<CognitionRequests> for Requests {
    fn from(r: CognitionRequests) -> Self {
        Self::Cognition(r)
    }
}
impl From<ConnectionRequests> for Requests {
    fn from(r: ConnectionRequests) -> Self {
        Self::Connection(r)
    }
}
impl From<DreamingRequests> for Requests {
    fn from(r: DreamingRequests) -> Self {
        Self::Dreaming(r)
    }
}
impl From<EventRequests> for Requests {
    fn from(r: EventRequests) -> Self {
        Self::Event(r)
    }
}
impl From<ExperienceRequests> for Requests {
    fn from(r: ExperienceRequests) -> Self {
        Self::Experience(r)
    }
}
impl From<IntrospectingRequests> for Requests {
    fn from(r: IntrospectingRequests) -> Self {
        Self::Introspecting(r)
    }
}
impl From<LevelRequests> for Requests {
    fn from(r: LevelRequests) -> Self {
        Self::Level(r)
    }
}
impl From<LifecycleRequests> for Requests {
    fn from(r: LifecycleRequests) -> Self {
        Self::Lifecycle(r)
    }
}
impl From<MemoryRequests> for Requests {
    fn from(r: MemoryRequests) -> Self {
        Self::Memory(r)
    }
}
impl From<NatureRequests> for Requests {
    fn from(r: NatureRequests) -> Self {
        Self::Nature(r)
    }
}
impl From<PersonaRequests> for Requests {
    fn from(r: PersonaRequests) -> Self {
        Self::Persona(r)
    }
}
impl From<ReflectingRequests> for Requests {
    fn from(r: ReflectingRequests) -> Self {
        Self::Reflecting(r)
    }
}
impl From<SearchRequests> for Requests {
    fn from(r: SearchRequests) -> Self {
        Self::Search(r)
    }
}
impl From<SensationRequests> for Requests {
    fn from(r: SensationRequests) -> Self {
        Self::Sensation(r)
    }
}
impl From<SenseRequests> for Requests {
    fn from(r: SenseRequests) -> Self {
        Self::Sense(r)
    }
}
impl From<StorageRequests> for Requests {
    fn from(r: StorageRequests) -> Self {
        Self::Storage(r)
    }
}
impl From<TextureRequests> for Requests {
    fn from(r: TextureRequests) -> Self {
        Self::Texture(r)
    }
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

impl From<AgentResponses> for Responses {
    fn from(r: AgentResponses) -> Self {
        Self::Agent(r)
    }
}
impl From<BrainResponses> for Responses {
    fn from(r: BrainResponses) -> Self {
        Self::Brain(r)
    }
}
impl From<CognitionResponses> for Responses {
    fn from(r: CognitionResponses) -> Self {
        Self::Cognition(r)
    }
}
impl From<ConnectionResponses> for Responses {
    fn from(r: ConnectionResponses) -> Self {
        Self::Connection(r)
    }
}
impl From<DreamingResponses> for Responses {
    fn from(r: DreamingResponses) -> Self {
        Self::Dreaming(r)
    }
}
impl From<EventResponses> for Responses {
    fn from(r: EventResponses) -> Self {
        Self::Event(r)
    }
}
impl From<ExperienceResponses> for Responses {
    fn from(r: ExperienceResponses) -> Self {
        Self::Experience(r)
    }
}
impl From<IntrospectingResponses> for Responses {
    fn from(r: IntrospectingResponses) -> Self {
        Self::Introspecting(r)
    }
}
impl From<LevelResponses> for Responses {
    fn from(r: LevelResponses) -> Self {
        Self::Level(r)
    }
}
impl From<LifecycleResponses> for Responses {
    fn from(r: LifecycleResponses) -> Self {
        Self::Lifecycle(r)
    }
}
impl From<MemoryResponses> for Responses {
    fn from(r: MemoryResponses) -> Self {
        Self::Memory(r)
    }
}
impl From<NatureResponses> for Responses {
    fn from(r: NatureResponses) -> Self {
        Self::Nature(r)
    }
}
impl From<PersonaResponses> for Responses {
    fn from(r: PersonaResponses) -> Self {
        Self::Persona(r)
    }
}
impl From<ReflectingResponses> for Responses {
    fn from(r: ReflectingResponses) -> Self {
        Self::Reflecting(r)
    }
}
impl From<SearchResponses> for Responses {
    fn from(r: SearchResponses) -> Self {
        Self::Search(r)
    }
}
impl From<SensationResponses> for Responses {
    fn from(r: SensationResponses) -> Self {
        Self::Sensation(r)
    }
}
impl From<SenseResponses> for Responses {
    fn from(r: SenseResponses) -> Self {
        Self::Sense(r)
    }
}
impl From<StorageResponses> for Responses {
    fn from(r: StorageResponses) -> Self {
        Self::Storage(r)
    }
}
impl From<TextureResponses> for Responses {
    fn from(r: TextureResponses) -> Self {
        Self::Texture(r)
    }
}
