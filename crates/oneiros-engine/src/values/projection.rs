use rusqlite::Connection;

use crate::StoredEvent;
use crate::event::EventError;

/// A projection — transforms events into read model state.
///
/// Each domain declares its projections. The event store runs them
/// after persisting each event. The projection's apply function
/// receives the database connection and the persisted event.
pub struct Projection {
    pub name: &'static str,
    pub apply: fn(&Connection, &StoredEvent) -> Result<(), EventError>,
    pub reset: fn(&Connection) -> Result<(), EventError>,
}
