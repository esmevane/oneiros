//! Response super-enum — collects all domain response types.
//!
//! Mirrors the Events and Requests super-enums. Enables unified
//! response handling across all transport layers.

use serde::{Deserialize, Serialize};

use crate::*;

/// All known response types across every domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Responses {
    Actor(ActorResponse),
    Agent(AgentResponse),
    Bookmark(BookmarkResponse),
    Cognition(CognitionResponse),
    Connection(ConnectionResponse),
    Continuity(ContinuityResponse),
    Doctor(DoctorResponse),
    Experience(ExperienceResponse),
    Follow(FollowResponse),
    Level(LevelResponse),
    Mcp(McpResponses),
    Memory(MemoryResponse),
    Nature(NatureResponse),
    Peer(PeerResponse),
    Persona(PersonaResponse),
    Pressure(PressureResponse),
    Project(ProjectResponse),
    Search(SearchResponse),
    Seed(SeedResponse),
    Sensation(SensationResponse),
    Slice(SliceResponse),
    Setup(SetupResponse),
    Storage(StorageResponse),
    Host(HostResponse),
    Lens(LensResponse),
    Tenant(TenantResponse),
    Texture(TextureResponse),
    Ticket(TicketResponse),
    Trail(TrailResponse),
    Urge(UrgeResponse),
}

collects_enum!(
    Responses::Actor => ActorResponse,
    Responses::Agent => AgentResponse,
    Responses::Bookmark => BookmarkResponse,
    Responses::Cognition => CognitionResponse,
    Responses::Connection => ConnectionResponse,
    Responses::Continuity => ContinuityResponse,
    Responses::Doctor => DoctorResponse,
    Responses::Experience => ExperienceResponse,
    Responses::Follow => FollowResponse,
    Responses::Level => LevelResponse,
    Responses::Mcp => McpResponses,
    Responses::Memory => MemoryResponse,
    Responses::Nature => NatureResponse,
    Responses::Peer => PeerResponse,
    Responses::Persona => PersonaResponse,
    Responses::Pressure => PressureResponse,
    Responses::Project => ProjectResponse,
    Responses::Search => SearchResponse,
    Responses::Seed => SeedResponse,
    Responses::Sensation => SensationResponse,
    Responses::Setup => SetupResponse,
    Responses::Slice => SliceResponse,
    Responses::Storage => StorageResponse,
    Responses::Host => HostResponse,
    Responses::Lens => LensResponse,
    Responses::Tenant => TenantResponse,
    Responses::Texture => TextureResponse,
    Responses::Ticket => TicketResponse,
    Responses::Trail => TrailResponse,
    Responses::Urge => UrgeResponse,
);
