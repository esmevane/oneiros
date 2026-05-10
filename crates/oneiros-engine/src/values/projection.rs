use crate::*;

/// A projection — transforms events into read model state.
///
/// Each projection owns its full lifecycle: schema creation (migrate),
/// event materialization (apply), and cleanup (reset). The event bus
/// orchestrates these; the projection owns the logic.
#[derive(Clone)]
pub(crate) struct Projection {
    pub(crate) name: &'static str,
    pub(crate) migrate: fn(&rusqlite::Connection) -> Result<(), EventError>,
    pub(crate) apply: fn(&rusqlite::Connection, &StoredEvent) -> Result<(), EventError>,
    pub(crate) reset: fn(&rusqlite::Connection) -> Result<(), EventError>,
}
