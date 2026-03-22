//! Typed event super-enum — compile-time safety for the event log.
//!
//! Each domain defines its own event enum. This module collects them into
//! a single `Events` type that the store persists. The `Unknown` variant
//! catches events from newer versions during replay.

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
    Unknown(serde_json::Value),
}

/// Extract the event type string from a serialized Events value.
///
/// The inner enums use `tag = "type"`, so the "type" field is always present
/// in the JSON representation of known events.
pub fn event_type(events: &Events) -> String {
    match serde_json::to_value(events) {
        Ok(v) => v
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
            .to_string(),
        Err(_) => "unknown".to_string(),
    }
}

// ── From impls ───────────────────────────────────────────────────

impl From<LevelEvents> for Events {
    fn from(e: LevelEvents) -> Self {
        Events::Level(e)
    }
}

impl From<TextureEvents> for Events {
    fn from(e: TextureEvents) -> Self {
        Events::Texture(e)
    }
}

impl From<SensationEvents> for Events {
    fn from(e: SensationEvents) -> Self {
        Events::Sensation(e)
    }
}

impl From<NatureEvents> for Events {
    fn from(e: NatureEvents) -> Self {
        Events::Nature(e)
    }
}

impl From<PersonaEvents> for Events {
    fn from(e: PersonaEvents) -> Self {
        Events::Persona(e)
    }
}

impl From<UrgeEvents> for Events {
    fn from(e: UrgeEvents) -> Self {
        Events::Urge(e)
    }
}

impl From<AgentEvents> for Events {
    fn from(e: AgentEvents) -> Self {
        Events::Agent(e)
    }
}

impl From<CognitionEvents> for Events {
    fn from(e: CognitionEvents) -> Self {
        Events::Cognition(e)
    }
}

impl From<MemoryEvents> for Events {
    fn from(e: MemoryEvents) -> Self {
        Events::Memory(e)
    }
}

impl From<ExperienceEvents> for Events {
    fn from(e: ExperienceEvents) -> Self {
        Events::Experience(e)
    }
}

impl From<ConnectionEvents> for Events {
    fn from(e: ConnectionEvents) -> Self {
        Events::Connection(e)
    }
}

impl From<StorageEvents> for Events {
    fn from(e: StorageEvents) -> Self {
        Events::Storage(e)
    }
}

impl From<ContinuityEvents> for Events {
    fn from(e: ContinuityEvents) -> Self {
        Events::Continuity(e)
    }
}

impl From<TenantEvents> for Events {
    fn from(e: TenantEvents) -> Self {
        Events::Tenant(e)
    }
}

impl From<ActorEvents> for Events {
    fn from(e: ActorEvents) -> Self {
        Events::Actor(e)
    }
}

impl From<BrainEvents> for Events {
    fn from(e: BrainEvents) -> Self {
        Events::Brain(e)
    }
}

impl From<TicketEvents> for Events {
    fn from(e: TicketEvents) -> Self {
        Events::Ticket(e)
    }
}
