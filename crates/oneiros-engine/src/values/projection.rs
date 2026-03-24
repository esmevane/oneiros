use rusqlite::Connection;

use crate::StoredEvent;
use crate::event::EventError;

/// A projection — transforms events into read model state.
///
/// Each projection owns its full lifecycle: schema creation (migrate),
/// event materialization (apply), and cleanup (reset). The event bus
/// orchestrates these; the projection owns the logic.
pub struct Projection {
    pub name: &'static str,
    pub migrate: fn(&Connection) -> Result<(), EventError>,
    pub apply: fn(&Connection, &StoredEvent) -> Result<(), EventError>,
    pub reset: fn(&Connection) -> Result<(), EventError>,
}
