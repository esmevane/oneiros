//! Project context — brain-scoped infrastructure.
//!
//! Carries the event bus, config, and source. All project-scoped
//! domain services receive this as their first argument.

use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::event::EventError;
use crate::event_bus::EventBus;
use crate::*;

/// The project-scoped application context.
///
/// All project-scoped domain services (Agent, Level, Cognition, etc.)
/// receive this. The bus handles event persistence and notification.
/// Projections run asynchronously via Frames.
#[derive(Clone)]
pub struct ProjectContext {
    bus: EventBus,
    frames: Frames,
    config: Option<Arc<Config>>,
    source: Source,
}

impl ProjectContext {
    pub fn new(bus: EventBus, frames: Frames) -> Self {
        Self {
            bus,
            frames,
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

    /// The server-level dream assembly configuration.
    pub fn dream_config(&self) -> DreamConfig {
        self.config
            .as_ref()
            .map(|c| c.dream.clone())
            .unwrap_or_default()
    }

    /// Execute a read operation against the database.
    ///
    /// Panics if the database mutex is poisoned (another thread panicked
    /// while holding the lock). This is an unrecoverable state.
    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        self.bus.with_db(f).expect("database lock poisoned")
    }

    /// Emit an event: append + dispatch + broadcast.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<StoredEvent, EventError> {
        self.bus.emit(event, self.source).await
    }

    /// Subscribe to the event broadcast (for SSE, dashboard).
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.bus.subscribe()
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(&self) -> Result<usize, EventError> {
        let events = self.bus.load_events()?;
        self.frames.replay(&events)
    }

    /// Access the event bus directly (for export/import operations).
    pub fn bus(&self) -> &EventBus {
        &self.bus
    }
}
