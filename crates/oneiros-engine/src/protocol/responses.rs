//! Response super-enum — collects all domain response types.
//!
//! Mirrors the Events and Requests super-enums. Enables unified
//! response handling across all transport layers.

use serde::{Deserialize, Serialize};

use crate::*;

/// All known response types across every domain.
#[expect(
    clippy::large_enum_variant,
    reason = "We can reduce the size of the ContinuityResponse later"
)]
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
    Continuity(ContinuityResponse),
    Pressure(PressureResponse),
    Search(SearchResponse),
    Project(ProjectResponse),
    Seed(SeedResponse),
    Doctor(DoctorResponse),
    System(SystemResponse),
    Tenant(TenantResponse),
    Actor(ActorResponse),
    Brain(BrainResponse),
    Ticket(TicketResponse),
    Bookmark(BookmarkResponse),
    Service(ServiceResponse),
    McpConfig(McpConfigResponse),
    Setup(SetupResponse),
}

collects_enum!(
    Responses::Level => LevelResponse,
    Responses::Texture => TextureResponse,
    Responses::Sensation => SensationResponse,
    Responses::Nature => NatureResponse,
    Responses::Persona => PersonaResponse,
    Responses::Urge => UrgeResponse,
    Responses::Agent => AgentResponse,
    Responses::Cognition => CognitionResponse,
    Responses::Memory => MemoryResponse,
    Responses::Experience => ExperienceResponse,
    Responses::Connection => ConnectionResponse,
    Responses::Storage => StorageResponse,
    Responses::Continuity => ContinuityResponse,
    Responses::Pressure => PressureResponse,
    Responses::Search => SearchResponse,
    Responses::Tenant => TenantResponse,
    Responses::Actor => ActorResponse,
    Responses::Brain => BrainResponse,
    Responses::Ticket => TicketResponse,
    Responses::Project => ProjectResponse,
    Responses::Service => ServiceResponse,
    Responses::Seed => SeedResponse,
    Responses::Doctor => DoctorResponse,
    Responses::System => SystemResponse,
    Responses::McpConfig => McpConfigResponse,
    Responses::Bookmark => BookmarkResponse,
    Responses::Setup => SetupResponse,
);
