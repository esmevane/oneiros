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
    Bookmark(BookmarkEvents),
    Ephemeral(EphemeralEvents),
    Unknown(serde_json::Value),
}

/// Ephemeral events — transport artifacts that are never persisted to the log.
///
/// These carry data between brains during export/import but are materialized
/// directly at the import boundary. They never enter the event log and are
/// never seen by projections.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum EphemeralEvents {
    /// Carries compressed blob binary for export/import portability.
    BlobStored(BlobContent),
}

impl Events {
    /// The kebab-case event type string, matching the serde `tag = "type"` value.
    ///
    /// This is the same string that appears in the JSON representation and
    /// is stored in the `event_type` column for query indexing.
    pub fn event_type(&self) -> &'static str {
        match self {
            Events::Level(e) => match e {
                LevelEvents::LevelSet(_) => "level-set",
                LevelEvents::LevelRemoved(_) => "level-removed",
            },
            Events::Texture(e) => match e {
                TextureEvents::TextureSet(_) => "texture-set",
                TextureEvents::TextureRemoved(_) => "texture-removed",
            },
            Events::Sensation(e) => match e {
                SensationEvents::SensationSet(_) => "sensation-set",
                SensationEvents::SensationRemoved(_) => "sensation-removed",
            },
            Events::Nature(e) => match e {
                NatureEvents::NatureSet(_) => "nature-set",
                NatureEvents::NatureRemoved(_) => "nature-removed",
            },
            Events::Persona(e) => match e {
                PersonaEvents::PersonaSet(_) => "persona-set",
                PersonaEvents::PersonaRemoved(_) => "persona-removed",
            },
            Events::Urge(e) => match e {
                UrgeEvents::UrgeSet(_) => "urge-set",
                UrgeEvents::UrgeRemoved(_) => "urge-removed",
            },
            Events::Agent(e) => match e {
                AgentEvents::AgentCreated(_) => "agent-created",
                AgentEvents::AgentUpdated(_) => "agent-updated",
                AgentEvents::AgentRemoved(_) => "agent-removed",
            },
            Events::Cognition(e) => match e {
                CognitionEvents::CognitionAdded(_) => "cognition-added",
            },
            Events::Memory(e) => match e {
                MemoryEvents::MemoryAdded(_) => "memory-added",
            },
            Events::Experience(e) => match e {
                ExperienceEvents::ExperienceCreated(_) => "experience-created",
                ExperienceEvents::ExperienceDescriptionUpdated(_) => {
                    "experience-description-updated"
                }
                ExperienceEvents::ExperienceSensationUpdated(_) => "experience-sensation-updated",
            },
            Events::Connection(e) => match e {
                ConnectionEvents::ConnectionCreated(_) => "connection-created",
                ConnectionEvents::ConnectionRemoved(_) => "connection-removed",
            },
            Events::Storage(e) => match e {
                StorageEvents::StorageSet(_) => "storage-set",
                StorageEvents::StorageRemoved(_) => "storage-removed",
            },
            Events::Continuity(e) => match e {
                ContinuityEvents::Dreamed(_) => "dreamed",
                ContinuityEvents::Introspected(_) => "introspected",
                ContinuityEvents::Reflected(_) => "reflected",
                ContinuityEvents::Sensed(_) => "sensed",
                ContinuityEvents::Slept(_) => "slept",
            },
            Events::Tenant(e) => match e {
                TenantEvents::TenantCreated(_) => "tenant-created",
            },
            Events::Actor(e) => match e {
                ActorEvents::ActorCreated(_) => "actor-created",
            },
            Events::Brain(e) => match e {
                BrainEvents::BrainCreated(_) => "brain-created",
            },
            Events::Ticket(e) => match e {
                TicketEvents::TicketIssued(_) => "ticket-issued",
            },
            Events::Bookmark(e) => match e {
                BookmarkEvents::BookmarkCreated(_) => "bookmark-created",
                BookmarkEvents::BookmarkForked(_) => "bookmark-forked",
                BookmarkEvents::BookmarkSwitched(_) => "bookmark-switched",
                BookmarkEvents::BookmarkMerged(_) => "bookmark-merged",
            },
            Events::Ephemeral(e) => match e {
                EphemeralEvents::BlobStored(_) => "blob-stored",
            },
            Events::Unknown(_) => "unknown",
        }
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

impl From<BookmarkEvents> for Events {
    fn from(e: BookmarkEvents) -> Self {
        Events::Bookmark(e)
    }
}

impl From<EphemeralEvents> for Events {
    fn from(e: EphemeralEvents) -> Self {
        Events::Ephemeral(e)
    }
}
