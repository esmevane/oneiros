//! Request super-enum — collects all domain request types.
//!
//! Mirrors the Events super-enum. Enables unified dispatch across
//! all transport layers (HTTP, MCP, CLI).

use serde::{Deserialize, Serialize};

use crate::*;

/// All known request types across every domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Requests {
    Actor(ActorRequest),
    Agent(AgentRequest),
    Brain(BrainRequest),
    Cognition(CognitionRequest),
    Connection(ConnectionRequest),
    Continuity(ContinuityRequest),
    Experience(ExperienceRequest),
    Level(LevelRequest),
    Memory(MemoryRequest),
    Nature(NatureRequest),
    Persona(PersonaRequest),
    Pressure(PressureRequest),
    Search(SearchRequest),
    Sensation(SensationRequest),
    Storage(StorageRequest),
    Tenant(TenantRequest),
    Texture(TextureRequest),
    Ticket(TicketRequest),
    Urge(UrgeRequest),
}

collects_enum!(
    Requests::Level => LevelRequest,
    Requests::Texture => TextureRequest,
    Requests::Sensation => SensationRequest,
    Requests::Nature => NatureRequest,
    Requests::Persona => PersonaRequest,
    Requests::Urge => UrgeRequest,
    Requests::Agent => AgentRequest,
    Requests::Cognition => CognitionRequest,
    Requests::Memory => MemoryRequest,
    Requests::Experience => ExperienceRequest,
    Requests::Connection => ConnectionRequest,
    Requests::Storage => StorageRequest,
    Requests::Continuity => ContinuityRequest,
    Requests::Pressure => PressureRequest,
    Requests::Search => SearchRequest,
    Requests::Tenant => TenantRequest,
    Requests::Actor => ActorRequest,
    Requests::Brain => BrainRequest,
    Requests::Ticket => TicketRequest,
);
