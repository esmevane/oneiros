//! Request super-enum — collects all domain request types.
//!
//! Mirrors the Events super-enum. Enables unified dispatch across
//! all transport layers (HTTP, MCP, CLI).

use serde::{Deserialize, Serialize};

use crate::*;

use crate::domains::agent::requests::AgentRequest;
use crate::domains::brain::requests::BrainRequest;
use crate::domains::cognition::requests::CognitionRequest;
use crate::domains::connection::requests::ConnectionRequest;
use crate::domains::experience::requests::ExperienceRequest;
use crate::domains::level::requests::LevelRequest;
use crate::domains::lifecycle::requests::LifecycleRequest;
use crate::domains::memory::requests::MemoryRequest;
use crate::domains::nature::requests::NatureRequest;
use crate::domains::persona::requests::PersonaRequest;
use crate::domains::pressure::requests::PressureRequest;
use crate::domains::search::requests::SearchRequest;
use crate::domains::sensation::requests::SensationRequest;
use crate::domains::storage::requests::StorageRequest;
use crate::domains::tenant::requests::TenantRequest;
use crate::domains::texture::requests::TextureRequest;
use crate::domains::ticket::requests::TicketRequest;
use crate::domains::urge::requests::UrgeRequest;

/// All known request types across every domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Requests {
    Level(LevelRequest),
    Texture(TextureRequest),
    Sensation(SensationRequest),
    Nature(NatureRequest),
    Persona(PersonaRequest),
    Urge(UrgeRequest),
    Agent(AgentRequest),
    Cognition(CognitionRequest),
    Memory(MemoryRequest),
    Experience(ExperienceRequest),
    Connection(ConnectionRequest),
    Storage(StorageRequest),
    Lifecycle(LifecycleRequest),
    Pressure(PressureRequest),
    Search(SearchRequest),
    Tenant(TenantRequest),
    Actor(ActorRequest),
    Brain(BrainRequest),
    Ticket(TicketRequest),
}

// ── From impls ───────────────────────────────────────────────────

impl From<LevelRequest> for Requests {
    fn from(r: LevelRequest) -> Self {
        Requests::Level(r)
    }
}
impl From<TextureRequest> for Requests {
    fn from(r: TextureRequest) -> Self {
        Requests::Texture(r)
    }
}
impl From<SensationRequest> for Requests {
    fn from(r: SensationRequest) -> Self {
        Requests::Sensation(r)
    }
}
impl From<NatureRequest> for Requests {
    fn from(r: NatureRequest) -> Self {
        Requests::Nature(r)
    }
}
impl From<PersonaRequest> for Requests {
    fn from(r: PersonaRequest) -> Self {
        Requests::Persona(r)
    }
}
impl From<UrgeRequest> for Requests {
    fn from(r: UrgeRequest) -> Self {
        Requests::Urge(r)
    }
}
impl From<AgentRequest> for Requests {
    fn from(r: AgentRequest) -> Self {
        Requests::Agent(r)
    }
}
impl From<CognitionRequest> for Requests {
    fn from(r: CognitionRequest) -> Self {
        Requests::Cognition(r)
    }
}
impl From<MemoryRequest> for Requests {
    fn from(r: MemoryRequest) -> Self {
        Requests::Memory(r)
    }
}
impl From<ExperienceRequest> for Requests {
    fn from(r: ExperienceRequest) -> Self {
        Requests::Experience(r)
    }
}
impl From<ConnectionRequest> for Requests {
    fn from(r: ConnectionRequest) -> Self {
        Requests::Connection(r)
    }
}
impl From<StorageRequest> for Requests {
    fn from(r: StorageRequest) -> Self {
        Requests::Storage(r)
    }
}
impl From<LifecycleRequest> for Requests {
    fn from(r: LifecycleRequest) -> Self {
        Requests::Lifecycle(r)
    }
}
impl From<PressureRequest> for Requests {
    fn from(r: PressureRequest) -> Self {
        Requests::Pressure(r)
    }
}
impl From<SearchRequest> for Requests {
    fn from(r: SearchRequest) -> Self {
        Requests::Search(r)
    }
}
impl From<TenantRequest> for Requests {
    fn from(r: TenantRequest) -> Self {
        Requests::Tenant(r)
    }
}
impl From<ActorRequest> for Requests {
    fn from(r: ActorRequest) -> Self {
        Requests::Actor(r)
    }
}
impl From<BrainRequest> for Requests {
    fn from(r: BrainRequest) -> Self {
        Requests::Brain(r)
    }
}
impl From<TicketRequest> for Requests {
    fn from(r: TicketRequest) -> Self {
        Requests::Ticket(r)
    }
}
