//! System context — system-scoped infrastructure.
//!
//! Carries the system database (tenants, actors, tickets, brains).
//! System-scoped domain services receive this as their first argument.

use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::events::Events;
use crate::store::{self, NewEvent, Projection, StoredEvent};

/// The system-scoped application context.
#[derive(Clone)]
pub struct SystemContext {
    db: Arc<Mutex<Connection>>,
    projections: &'static [&'static [Projection]],
    events: broadcast::Sender<StoredEvent>,
    source: String,
}

impl SystemContext {
    pub fn new(conn: Connection, projections: &'static [&'static [Projection]]) -> Self {
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

    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.db.lock().expect("db lock");
        f(&conn)
    }

    pub fn emit(&self, event: impl Into<Events>) -> StoredEvent {
        let new_event = NewEvent {
            data: event.into(),
            source: self.source.clone(),
        };

        let stored = self.with_db(|conn| {
            store::log_event(conn, &new_event, self.projections).expect("log event")
        });

        let _ = self.events.send(stored.clone());
        stored
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.events.subscribe()
    }
}
