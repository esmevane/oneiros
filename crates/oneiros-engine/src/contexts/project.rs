//! Project context — brain-scoped infrastructure.
//!
//! Carries the brain database, projections, event broadcast, config, and source.
//! All project-scoped domain services receive this as their first argument.

use rusqlite::Connection;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use crate::*;

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
    config: Option<Arc<Config>>,
    source: Source,
}

impl ProjectContext {
    pub fn new(conn: Connection, projections: &'static [&'static [Projection]]) -> Self {
        let (events, _) = broadcast::channel(256);
        Self {
            db: Arc::new(Mutex::new(conn)),
            projections,
            events,
            config: None,
            source: Source::default(),
        }
    }

    /// Construct an HTTP client from the config's service address.
    pub fn client(&self) -> Client {
        let base_url = self
            .config
            .as_ref()
            .map(|c| c.base_url())
            .unwrap_or_else(|| "http://127.0.0.1:2100".to_string());

        Client::new(base_url)
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source = source;
        self
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(Arc::new(config));
        self
    }

    /// The data directory for filesystem operations (blobs, exports).
    pub fn data_dir(&self) -> Option<&Path> {
        self.config.as_ref().map(|c| c.data_dir.as_path())
    }

    /// Execute a read operation against the database.
    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.db.lock().expect("db lock");
        f(&conn)
    }

    /// Emit an event: persist + run projections + broadcast.
    pub fn emit(&self, event: impl Into<Events>) -> StoredEvent {
        let new_event = NewEvent {
            data: event.into(),
            source: self.source,
        };

        let stored = self.with_db(|conn| {
            repo::log_event(conn, &new_event, self.projections).expect("log event")
        });

        let _ = self.events.send(stored.clone());
        stored
    }

    /// Subscribe to the event broadcast.
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.events.subscribe()
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(&self) -> Result<usize, Box<dyn std::error::Error>> {
        self.with_db(|conn| repo::replay(conn, self.projections).map_err(|e| e.into()))
    }
}
