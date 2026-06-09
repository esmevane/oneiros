//! Typed event super-enum — compile-time safety for the event log.
//!
//! Each domain defines its own event enum. This module collects them into
//! a single `Events` type that the store persists. Events rows from disk
//! that fail to parse as `Events` are captured by the `EventRecord`
//! envelope as `Unknown`, where they can be logged and filtered without
//! contaminating the pure `Events` type.

use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

/// All known event types across every domain.
///
/// Uses `#[serde(untagged)]` so the JSON representation is just the inner
/// enum's tagged form (e.g. `{"type": "level-set", "data": {...}}`).
/// Every variant is a real comprehended event — unrecognized shapes are
/// held by `EventRecord::Unknown` at the envelope level, never here.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Events {
    Level(LevelEvents),
    Texture(TextureEvents),
    Sensation(SensationEvents),
    Slice(SliceEvents),
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
    Project(ProjectEvents),
    Ticket(TicketEvents),
    Bookmark(BookmarkEvents),
    Peer(PeerEvents),
    Remote(RemoteEvents),
}

impl Events {
    /// The kebab-case event type string, matching the serde `tag = "type"` value.
    ///
    /// This is the same string that appears in the JSON representation and
    /// is stored in the `event_type` column for query indexing.
    pub(crate) fn event_type(&self) -> String {
        match self {
            Events::Level(e) => e.kind().to_string(),
            Events::Texture(e) => e.kind().to_string(),
            Events::Sensation(e) => e.kind().to_string(),
            Events::Slice(e) => e.kind().to_string(),
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
            Events::Project(e) => e.kind().to_string(),
            Events::Ticket(e) => e.kind().to_string(),
            Events::Bookmark(e) => e.kind().to_string(),
            Events::Peer(e) => e.kind().to_string(),
            Events::Remote(e) => e.kind().to_string(),
        }
    }
}

collects_enum!(
    Events::Level => LevelEvents,
    Events::Texture => TextureEvents,
    Events::Sensation => SensationEvents,
    Events::Slice => SliceEvents,
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
    Events::Project => ProjectEvents,
    Events::Ticket => TicketEvents,
    Events::Bookmark => BookmarkEvents,
    Events::Peer => PeerEvents,
    Events::Remote => RemoteEvents,
);

/// Ephemeral events — transport artifacts that are never persisted to the log.
///
/// These carry data between projects during export/import but are materialized
/// directly at the import boundary. They never enter the event log and are
/// never seen by projections.
#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = EphemeralEventsType, display = "kebab-case")]
pub(crate) enum EphemeralEvents {
    /// Carries compressed blob binary for export/import portability.
    BlobStored(BlobContent),
}
