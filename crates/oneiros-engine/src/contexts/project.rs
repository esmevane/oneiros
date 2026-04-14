use tokio::sync::broadcast;

use crate::*;

#[derive(Clone)]
pub(crate) struct ProjectContext {
    pub(crate) config: Config,
    bus: Option<EventBus>,
}

impl ProjectContext {
    /// Create a client-only context — can build HTTP clients but cannot emit.
    pub(crate) fn new(config: Config) -> Self {
        Self { config, bus: None }
    }

    /// Create a context backed by an EventBus.
    pub(crate) fn with_bus(config: Config, bus: EventBus) -> Self {
        Self {
            config,
            bus: Some(bus),
        }
    }

    /// The brain name for this project.
    pub(crate) fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// Build an authenticated HTTP client for this project.
    pub(crate) fn client(&self) -> Client {
        match self.config.token() {
            Some(token) => Client::with_token(self.config.base_url(), token),
            None => Client::new(self.config.base_url()),
        }
    }

    /// Subscribe to the event broadcast stream.
    pub(crate) fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.bus
            .as_ref()
            .expect("subscribe requires a running bus")
            .subscribe()
    }

    /// Open the brain (project) database.
    pub(crate) fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.brain_db()
    }

    /// Emit an event through the bus.
    ///
    /// Appends to the event log and broadcasts to subscribers.
    /// Projectors pick up the broadcast and apply asynchronously.
    pub(crate) async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let bus = self
            .bus
            .as_ref()
            .ok_or_else(|| EventError::Lock("emit requires a running bus".to_string()))?;

        bus.emit(event, Source::default())?;

        Ok(())
    }
}
