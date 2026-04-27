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

/// Wire-level envelope for events read from persistence.
///
/// A row in the events table is parsed as `EventRecord`. Rows whose
/// type tag is recognized become `Known(Events)`; rows whose shape
/// doesn't match any known variant become `Unknown` with the original
/// JSON preserved for diagnostics. Callers at the load boundary filter
/// Unknown entries (typically with a warn-level log) so downstream
/// code only ever handles pure `Events`.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Event {
    Known(Events),
    Ephemeral(EphemeralEvents),
    Unknown(UnknownEvent),
    #[default]
    Malformed,
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Self::Known(known) => known.event_type(),
            Self::Ephemeral(ephemeral) => ephemeral.kind().to_string(),
            Self::Unknown(_) => "__@unknown".to_string(),
            Self::Malformed => "__@malformed".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct UnknownEvent {
    #[serde(rename = "type")]
    pub type_tag: String,
    pub data: serde_json::Value,
}

/// All known event types across every domain.
///
/// Uses `#[serde(untagged)]` so the JSON representation is just the inner
/// enum's tagged form (e.g. `{"type": "level-set", "data": {...}}`).
/// Every variant is a real comprehended event — unrecognized shapes are
/// held by `EventRecord::Unknown` at the envelope level, never here.
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
);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_event_round_trips_through_event_record() {
        let level = Level::Current(
            Level::build_v1()
                .name("working")
                .description("Short-term")
                .prompt("")
                .build(),
        );
        let original = Events::Level(LevelEvents::LevelSet(level));

        let json = serde_json::to_string(&original).unwrap();
        let record: Event = serde_json::from_str(&json).unwrap();

        match record {
            Event::Known(Events::Level(LevelEvents::LevelSet(_))) => {}
            other => panic!("expected Known LevelSet, got {other:?}"),
        }
    }

    #[test]
    fn unrecognized_type_tag_becomes_unknown_with_tag_preserved() {
        let raw = r#"{"type": "future-event", "data": {"anything": 42}}"#;
        let record: Event = serde_json::from_str(raw).unwrap();

        match record {
            Event::Unknown(UnknownEvent { type_tag, data }) => {
                assert_eq!(type_tag, "future-event");
                assert_eq!(data["anything"], 42);
            }
            other => panic!("expected Unknown, got {other:?}"),
        }
    }

    #[test]
    fn type_tag_defaults_to_empty_when_missing() {
        let raw = r#"{"type": "flargunnstow", "data": {"nothing": "here"}}"#;
        let record: Event = serde_json::from_str(raw).unwrap();

        match record {
            Event::Unknown(UnknownEvent { type_tag, .. }) => assert_eq!(type_tag, "flargunnstow"),
            other => panic!("expected Unknown, got {other:?}"),
        }
    }

    #[test]
    fn known_event_record_serializes_transparently() {
        let level = Level::Current(
            Level::build_v1()
                .name("working")
                .description("Short-term")
                .prompt("")
                .build(),
        );
        let record = Event::Known(Events::Level(LevelEvents::LevelSet(level.clone())));
        let record_json = serde_json::to_value(&record).unwrap();

        let direct = Events::Level(LevelEvents::LevelSet(level));
        let direct_json = serde_json::to_value(&direct).unwrap();

        assert_eq!(record_json, direct_json);
    }

    #[test]
    fn unknown_event_record_serializes_raw_data() {
        let data = serde_json::json!({"type": "future-event", "data": {"x": 1}});
        let record = Event::Unknown(UnknownEvent {
            type_tag: "future-event".to_string(),
            data: serde_json::json!({ "x": 1 }),
        });

        assert_eq!(serde_json::to_value(&record).unwrap(), data);
    }
}
