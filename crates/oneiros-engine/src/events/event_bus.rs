//! EventBus — append and broadcast.
//!
//! The bus owns event persistence and notification. Two steps:
//!
//! 1. **Append** — persist to the EventLog (durable fact)
//! 2. **Broadcast** — notify all subscribers (projectors, SSE, peers)
//!
//! The bus does NOT run projections. Projectors subscribe to the
//! broadcast and apply events at their own pace. Reads that need
//! fresh projection state use eventual-consistency retry patterns.

use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::broadcast;

use crate::*;

/// The event bus — append and broadcast.
#[derive(Clone)]
pub(crate) struct EventBus {
    db: Arc<Mutex<rusqlite::Connection>>,
    broadcast: broadcast::Sender<StoredEvent>,
}

/// Acquire the database lock, converting PoisonError to EventError.
fn lock(
    db: &Mutex<rusqlite::Connection>,
) -> Result<MutexGuard<'_, rusqlite::Connection>, EventError> {
    db.lock().map_err(|e| EventError::Lock(e.to_string()))
}

impl EventBus {
    /// Create a new bus.
    pub(crate) fn new(db: Arc<Mutex<rusqlite::Connection>>) -> Self {
        let (broadcast, _) = broadcast::channel(256);
        Self { db, broadcast }
    }

    /// Publish an event: append to log, then broadcast.
    ///
    /// Returns the stored event after it has been durably persisted.
    /// Broadcast is fire-and-forget — subscribers process asynchronously.
    pub(crate) fn publish(&self, event: NewEvent) -> Result<StoredEvent, EventError> {
        let stored = {
            let conn = lock(&self.db)?;
            EventLog::new(&conn).append(&event)?
        };

        let _ = self.broadcast.send(stored.clone());

        Ok(stored)
    }

    /// Emit an event with a given source: construct + publish.
    pub(crate) fn emit(
        &self,
        event: impl Into<Events>,
        source: Source,
    ) -> Result<StoredEvent, EventError> {
        let new_event = NewEvent {
            data: event.into(),
            source,
        };
        self.publish(new_event)
    }

    /// Subscribe to the broadcast channel.
    pub(crate) fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.broadcast.subscribe()
    }

    /// The broadcast sender for SSE subscribers.
    pub(crate) fn broadcast(&self) -> &broadcast::Sender<StoredEvent> {
        &self.broadcast
    }

    /// Execute a read operation against the database.
    pub(crate) fn with_db<T>(
        &self,
        f: impl FnOnce(&rusqlite::Connection) -> T,
    ) -> Result<T, EventError> {
        let conn = lock(&self.db)?;
        Ok(f(&conn))
    }

    /// Load all events from the log (for export, replay source).
    pub(crate) fn load_events(&self) -> Result<Vec<StoredEvent>, EventError> {
        let conn = lock(&self.db)?;
        EventLog::new(&conn).load_all()
    }

    /// Import an event to the log without broadcasting.
    ///
    /// For bulk import. After import, projectors should replay
    /// from the event log to rebuild read models.
    pub(crate) fn import(&self, event: &StoredEvent) -> Result<(), EventError> {
        let conn = lock(&self.db)?;
        EventLog::new(&conn).import(event)
    }
}
