use crate::*;

/// A projection — transforms events into read model state.
///
/// Each projection owns its full lifecycle: schema creation (migrate),
/// event materialization (apply), and cleanup (reset). The event bus
/// orchestrates these; the projection owns the logic.
#[derive(Clone)]
pub struct Projection {
    pub name: &'static str,
    pub migrate: fn(&rusqlite::Connection) -> Result<(), EventError>,
    pub apply: fn(&rusqlite::Connection, &StoredEvent) -> Result<(), EventError>,
    pub reset: fn(&rusqlite::Connection) -> Result<(), EventError>,
}
