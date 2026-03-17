//! Ports — the application context that all layers receive.
//!
//! In the ducks model, the "port" is simply the AppContext — it wraps
//! the database behind a mutex and provides emit() for event persistence
//! + broadcast. No trait abstraction needed; the Mutex IS the adapter.

use oneiros_db::{Database, Projection};
use oneiros_model::*;
use tokio::sync::broadcast;

/// Application context — the shared state that all layers receive.
///
/// This is the wiring: database behind a mutex, projections, event bus,
/// and source identity. Driving adapters (HTTP, MCP, CLI) receive this
/// via axum State or direct injection.
#[derive(Clone)]
pub struct AppContext {
    db: std::sync::Arc<std::sync::Mutex<Database>>,
    projections: &'static [&'static [Projection]],
    events: broadcast::Sender<Event>,
    source: Source,
}

impl AppContext {
    pub fn new(
        db: Database,
        projections: &'static [&'static [Projection]],
    ) -> Self {
        let (events, _) = broadcast::channel(256);
        Self {
            db: std::sync::Arc::new(std::sync::Mutex::new(db)),
            projections,
            events,
            source: Source::default(),
        }
    }

    /// Execute a read operation against the database.
    pub fn with_db<T>(&self, f: impl FnOnce(&Database) -> T) -> T {
        let db = self.db.lock().expect("db lock");
        f(&db)
    }

    /// Emit an event: persist + run projections + broadcast.
    pub fn emit(&self, event_data: Events) -> Event {
        let new_event = NewEvent::new(event_data, self.source);
        let persisted = self.with_db(|db| {
            db.log_event(&new_event, self.projections).expect("log event")
        });
        let _ = self.events.send(persisted.clone());
        persisted
    }

    /// Subscribe to the event broadcast.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.events.subscribe()
    }
}
