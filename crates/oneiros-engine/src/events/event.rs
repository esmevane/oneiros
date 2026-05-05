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
///
/// This enum describes on-disk row shapes only — bus traffic uses
/// concrete `NewEvent` / `StoredEvent` types directly via per-actor
/// message protocols.
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
