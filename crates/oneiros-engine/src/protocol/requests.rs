//! Request super-enum — collects all domain request types.
//!
//! Mirrors the Events super-enum. Enables unified dispatch across
//! all transport layers (HTTP, MCP, CLI).

use serde::{Deserialize, Serialize};

use crate::*;

/// All known request types across every domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[expect(dead_code, reason = "This will be important to streaming")]
pub(crate) enum Requests {
    Actor(ActorRequest),
    Agent(AgentRequest),
    Bookmark(BookmarkRequest),
    Cognition(CognitionRequest),
    Connection(ConnectionRequest),
    Continuity(ContinuityRequest),
    Experience(ExperienceRequest),
    Follow(FollowRequest),
    Level(LevelRequest),
    Memory(MemoryRequest),
    Nature(NatureRequest),
    Peer(PeerRequest),
    Persona(PersonaRequest),
    Pressure(PressureRequest),
    Project(ProjectRequest),
    Search(SearchRequest),
    Seed(SeedRequest),
    Sensation(SensationRequest),
    Storage(StorageRequest),
    Host(HostRequest),
    Tenant(TenantRequest),
    Texture(TextureRequest),
    Ticket(TicketRequest),
    Trail(TrailRequest),
    Urge(UrgeRequest),
}

collects_enum!(
    Requests::Actor => ActorRequest,
    Requests::Agent => AgentRequest,
    Requests::Bookmark => BookmarkRequest,
    Requests::Cognition => CognitionRequest,
    Requests::Connection => ConnectionRequest,
    Requests::Continuity => ContinuityRequest,
    Requests::Experience => ExperienceRequest,
    Requests::Follow => FollowRequest,
    Requests::Level => LevelRequest,
    Requests::Memory => MemoryRequest,
    Requests::Nature => NatureRequest,
    Requests::Project => ProjectRequest,
    Requests::Peer => PeerRequest,
    Requests::Persona => PersonaRequest,
    Requests::Pressure => PressureRequest,
    Requests::Search => SearchRequest,
    Requests::Seed => SeedRequest,
    Requests::Sensation => SensationRequest,
    Requests::Storage => StorageRequest,
    Requests::Host => HostRequest,
    Requests::Tenant => TenantRequest,
    Requests::Texture => TextureRequest,
    Requests::Ticket => TicketRequest,
    Requests::Trail => TrailRequest,
    Requests::Urge => UrgeRequest,
);
