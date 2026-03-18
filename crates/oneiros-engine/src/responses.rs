//! Response super-enum — collects all domain response types.
//!
//! Mirrors the Events and Requests super-enums. Enables unified
//! response handling across all transport layers.

use serde::{Deserialize, Serialize};

use crate::*;

use crate::domains::agent::responses::AgentResponse;
use crate::domains::brain::responses::BrainResponse;
use crate::domains::cognition::responses::CognitionResponse;
use crate::domains::connection::responses::ConnectionResponse;
use crate::domains::experience::responses::ExperienceResponse;
use crate::domains::level::responses::LevelResponse;
use crate::domains::lifecycle::responses::LifecycleResponse;
use crate::domains::memory::responses::MemoryResponse;
use crate::domains::nature::responses::NatureResponse;
use crate::domains::persona::responses::PersonaResponse;
use crate::domains::pressure::responses::PressureResponse;
use crate::domains::search::responses::SearchResponse;
use crate::domains::sensation::responses::SensationResponse;
use crate::domains::storage::responses::StorageResponse;
use crate::domains::tenant::responses::TenantResponse;
use crate::domains::texture::responses::TextureResponse;
use crate::domains::ticket::responses::TicketResponse;
use crate::domains::urge::responses::UrgeResponse;

/// All known response types across every domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Responses {
    Level(LevelResponse),
    Texture(TextureResponse),
    Sensation(SensationResponse),
    Nature(NatureResponse),
    Persona(PersonaResponse),
    Urge(UrgeResponse),
    Agent(AgentResponse),
    Cognition(CognitionResponse),
    Memory(MemoryResponse),
    Experience(ExperienceResponse),
    Connection(ConnectionResponse),
    Storage(StorageResponse),
    Lifecycle(LifecycleResponse),
    Pressure(PressureResponse),
    Search(SearchResponse),
    Tenant(TenantResponse),
    Actor(ActorResponse),
    Brain(BrainResponse),
    Ticket(TicketResponse),
}

// ── From impls ───────────────────────────────────────────────────

impl From<LevelResponse> for Responses {
    fn from(r: LevelResponse) -> Self {
        Responses::Level(r)
    }
}
impl From<TextureResponse> for Responses {
    fn from(r: TextureResponse) -> Self {
        Responses::Texture(r)
    }
}
impl From<SensationResponse> for Responses {
    fn from(r: SensationResponse) -> Self {
        Responses::Sensation(r)
    }
}
impl From<NatureResponse> for Responses {
    fn from(r: NatureResponse) -> Self {
        Responses::Nature(r)
    }
}
impl From<PersonaResponse> for Responses {
    fn from(r: PersonaResponse) -> Self {
        Responses::Persona(r)
    }
}
impl From<UrgeResponse> for Responses {
    fn from(r: UrgeResponse) -> Self {
        Responses::Urge(r)
    }
}
impl From<AgentResponse> for Responses {
    fn from(r: AgentResponse) -> Self {
        Responses::Agent(r)
    }
}
impl From<CognitionResponse> for Responses {
    fn from(r: CognitionResponse) -> Self {
        Responses::Cognition(r)
    }
}
impl From<MemoryResponse> for Responses {
    fn from(r: MemoryResponse) -> Self {
        Responses::Memory(r)
    }
}
impl From<ExperienceResponse> for Responses {
    fn from(r: ExperienceResponse) -> Self {
        Responses::Experience(r)
    }
}
impl From<ConnectionResponse> for Responses {
    fn from(r: ConnectionResponse) -> Self {
        Responses::Connection(r)
    }
}
impl From<StorageResponse> for Responses {
    fn from(r: StorageResponse) -> Self {
        Responses::Storage(r)
    }
}
impl From<LifecycleResponse> for Responses {
    fn from(r: LifecycleResponse) -> Self {
        Responses::Lifecycle(r)
    }
}
impl From<PressureResponse> for Responses {
    fn from(r: PressureResponse) -> Self {
        Responses::Pressure(r)
    }
}
impl From<SearchResponse> for Responses {
    fn from(r: SearchResponse) -> Self {
        Responses::Search(r)
    }
}
impl From<TenantResponse> for Responses {
    fn from(r: TenantResponse) -> Self {
        Responses::Tenant(r)
    }
}
impl From<ActorResponse> for Responses {
    fn from(r: ActorResponse) -> Self {
        Responses::Actor(r)
    }
}
impl From<BrainResponse> for Responses {
    fn from(r: BrainResponse) -> Self {
        Responses::Brain(r)
    }
}
impl From<TicketResponse> for Responses {
    fn from(r: TicketResponse) -> Self {
        Responses::Ticket(r)
    }
}
