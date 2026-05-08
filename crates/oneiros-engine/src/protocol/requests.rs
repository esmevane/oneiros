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
    Bookmark(BookmarkRequest),
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
    Requests::Actor => ActorRequest,
    Requests::Agent => AgentRequest,
    Requests::Bookmark => BookmarkRequest,
    Requests::Brain => BrainRequest,
    Requests::Cognition => CognitionRequest,
    Requests::Connection => ConnectionRequest,
    Requests::Continuity => ContinuityRequest,
    Requests::Experience => ExperienceRequest,
    Requests::Level => LevelRequest,
    Requests::Memory => MemoryRequest,
    Requests::Nature => NatureRequest,
    Requests::Persona => PersonaRequest,
    Requests::Pressure => PressureRequest,
    Requests::Search => SearchRequest,
    Requests::Sensation => SensationRequest,
    Requests::Storage => StorageRequest,
    Requests::Tenant => TenantRequest,
    Requests::Texture => TextureRequest,
    Requests::Ticket => TicketRequest,
    Requests::Urge => UrgeRequest,
);
