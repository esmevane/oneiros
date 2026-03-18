//! Project context — brain-scoped infrastructure.
//!
//! Carries the brain database, projections, event broadcast, and source.
//! All project-scoped domain services receive this as their first argument.

use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::store::{self, NewEvent, Projection, StoredEvent, StoreError};

/// The project-scoped application context.
///
/// All project-scoped domain services (Agent, Level, Cognition, etc.)
/// receive this. It wraps the brain database behind a mutex, holds
/// the collected projections, and provides the event bus.
#[derive(Clone)]
pub struct ProjectContext {
    db: Arc<Mutex<Connection>>,
    projections: &'static [&'static [Projection]],
    events: broadcast::Sender<StoredEvent>,
    source: String,
}

impl ProjectContext {
    pub fn new(
        conn: Connection,
        projections: &'static [&'static [Projection]],
    ) -> Self {
        let (events, _) = broadcast::channel(256);
        Self {
            db: Arc::new(Mutex::new(conn)),
            projections,
            events,
            source: String::new(),
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Execute a read operation against the database.
    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.db.lock().expect("db lock");
        f(&conn)
    }

    /// Emit an event: persist + run projections + broadcast.
    pub fn emit(&self, event_type: &str, data: &impl serde::Serialize) -> StoredEvent {
        let data_value = serde_json::to_value(data).expect("serialize event data");
        let new_event = NewEvent {
            event_type: event_type.to_string(),
            data: data_value,
            source: self.source.clone(),
        };

        let stored = self.with_db(|conn| {
            store::log_event(conn, &new_event, self.projections).expect("log event")
        });

        let _ = self.events.send(stored.clone());
        stored
    }

    /// Subscribe to the event broadcast.
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.events.subscribe()
    }
}
