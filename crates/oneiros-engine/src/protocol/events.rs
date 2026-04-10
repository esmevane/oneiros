//! Typed event super-enum — compile-time safety for the event log.
//!
//! Each domain defines its own event enum. This module collects them into
//! a single `Events` type that the store persists. The `Unknown` variant
//! catches events from newer versions during replay.

use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

/// All known event types across every domain.
///
/// Uses `#[serde(untagged)]` so the JSON representation is just the inner
/// enum's tagged form (e.g. `{"type": "level-set", "data": {...}}`).
/// The `Unknown` variant must be last — it catches anything that doesn't
/// match a known domain event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Level(LevelEvents),
    Texture(TextureEvents),
    Sensation(SensationEvents),
    Nature(NatureEvents),
    Persona(PersonaEvents),
    Urge(UrgeEvents),
    Agent(AgentEvents),
    Cognition(CognitionEvents),
    Memory(MemoryEvents),
    Experience(ExperienceEvents),
    Connection(ConnectionEvents),
    Storage(StorageEvents),
    Continuity(ContinuityEvents),
    Tenant(TenantEvents),
    Actor(ActorEvents),
    Brain(BrainEvents),
    Ticket(TicketEvents),
    Bookmark(BookmarkEvents),
    Peer(PeerEvents),
    Ephemeral(EphemeralEvents),
    Unknown(serde_json::Value),
}

/// Ephemeral events — transport artifacts that are never persisted to the log.
///
/// These carry data between brains during export/import but are materialized
/// directly at the import boundary. They never enter the event log and are
/// never seen by projections.
#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = EphemeralEventsType, display = "kebab-case")]
pub enum EphemeralEvents {
    /// Carries compressed blob binary for export/import portability.
    BlobStored(BlobContent),
}

impl Events {
    /// The kebab-case event type string, matching the serde `tag = "type"` value.
    ///
    /// This is the same string that appears in the JSON representation and
    /// is stored in the `event_type` column for query indexing.
    pub fn event_type(&self) -> String {
        match self {
            Events::Level(e) => e.kind().to_string(),
            Events::Texture(e) => e.kind().to_string(),
            Events::Sensation(e) => e.kind().to_string(),
            Events::Nature(e) => e.kind().to_string(),
            Events::Persona(e) => e.kind().to_string(),
            Events::Urge(e) => e.kind().to_string(),
            Events::Agent(e) => e.kind().to_string(),
            Events::Cognition(e) => e.kind().to_string(),
            Events::Memory(e) => e.kind().to_string(),
            Events::Experience(e) => e.kind().to_string(),
            Events::Connection(e) => e.kind().to_string(),
            Events::Storage(e) => e.kind().to_string(),
            Events::Continuity(e) => e.kind().to_string(),
            Events::Tenant(e) => e.kind().to_string(),
            Events::Actor(e) => e.kind().to_string(),
            Events::Brain(e) => e.kind().to_string(),
            Events::Ticket(e) => e.kind().to_string(),
            Events::Bookmark(e) => e.kind().to_string(),
            Events::Peer(e) => e.kind().to_string(),
            Events::Ephemeral(e) => e.kind().to_string(),
            Events::Unknown(_) => "unknown".to_string(),
        }
    }
}

collects_enum!(
    Events::Level => LevelEvents,
    Events::Texture => TextureEvents,
    Events::Sensation => SensationEvents,
    Events::Nature => NatureEvents,
    Events::Persona => PersonaEvents,
    Events::Urge => UrgeEvents,
    Events::Agent => AgentEvents,
    Events::Cognition => CognitionEvents,
    Events::Memory => MemoryEvents,
    Events::Experience => ExperienceEvents,
    Events::Connection => ConnectionEvents,
    Events::Storage => StorageEvents,
    Events::Continuity => ContinuityEvents,
    Events::Tenant => TenantEvents,
    Events::Actor => ActorEvents,
    Events::Brain => BrainEvents,
    Events::Ticket => TicketEvents,
    Events::Bookmark => BookmarkEvents,
    Events::Peer => PeerEvents,
    Events::Ephemeral => EphemeralEvents,
);
