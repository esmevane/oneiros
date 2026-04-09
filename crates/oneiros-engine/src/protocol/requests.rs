//! Request super-enum — collects all domain request types.
//!
//! Mirrors the Events super-enum. Enables unified dispatch across
//! all transport layers (HTTP, MCP, CLI).

use serde::{Deserialize, Serialize};

use crate::*;

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
    Continuity(ContinuityRequest),
    Pressure(PressureRequest),
    Search(SearchRequest),
    Tenant(TenantRequest),
    Actor(ActorRequest),
    Brain(BrainRequest),
    Ticket(TicketRequest),
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
