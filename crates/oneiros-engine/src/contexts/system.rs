//! System context — system-scoped infrastructure.
//!
//! Carries the system database (tenants, actors, tickets, brains).
//! System-scoped domain services receive this as their first argument.

use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::store::{self, NewEvent, Projection, StoredEvent};

/// The system-scoped application context.
#[derive(Clone)]
pub struct SystemContext {
    db: Arc<Mutex<Connection>>,
    projections: &'static [&'static [Projection]],
    events: broadcast::Sender<StoredEvent>,
}

impl SystemContext {
    pub fn new(conn: Connection, projections: &'static [&'static [Projection]]) -> Self {
        let (events, _) = broadcast::channel(256);
        Self {
            db: Arc::new(Mutex::new(conn)),
            projections,
            events,
        }
    }

    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.db.lock().expect("db lock");
        f(&conn)
    }

    pub fn emit(&self, event_type: &str, data: &impl serde::Serialize) -> StoredEvent {
        let data_value = serde_json::to_value(data).expect("serialize event data");
        let new_event = NewEvent {
            event_type: event_type.to_string(),
            data: data_value,
            source: String::new(),
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
