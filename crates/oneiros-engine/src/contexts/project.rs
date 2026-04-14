use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

use crate::*;

#[derive(Clone)]
pub(crate) struct ProjectContext {
    pub(crate) config: Config,
    bus: Option<EventBus>,
}

impl ProjectContext {
    /// Create a client-only context — can build HTTP clients but cannot emit.
    ///
    /// Used by CLI commands that route through HTTP. For contexts that
    /// need to emit events directly, use `start()` or construct via
    /// `ServerState::project_context()`.
    pub(crate) fn new(config: Config) -> Self {
        Self { config, bus: None }
    }

    /// Create a context backed by a running EventBus.
    ///
    /// Used by the server to hand each request a context that emits
    /// through the shared bus.
    pub(crate) fn with_bus(config: Config, bus: EventBus) -> Self {
        Self {
            config,
            bus: Some(bus),
        }
    }

    /// Create a self-contained context with its own bus and consumer.
    ///
    /// Opens the brain database, runs migrations, and spawns a consumer
    /// task. Useful for tests and non-server consumers that need to
    /// emit events without an HTTP server.
    pub(crate) fn start(config: Config) -> Result<Self, EventError> {
        let db = config.brain_db()?;

        EventLog::new(&db).migrate()?;

        let projections = Projections::project();
        projections.migrate(&db)?;

        let chronicle = Chronicle::new();
        let db = Arc::new(Mutex::new(db));
        let bus = EventBus::spawn(db, projections, chronicle);

        Ok(Self {
            config,
            bus: Some(bus),
        })
    }

    /// The brain name for this project.
    pub(crate) fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// Build an authenticated HTTP client for this project.
    ///
    /// Reads the token from the token file on disk. Returns an
    /// unauthenticated client if no token file exists yet (e.g. before
    /// project init).
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

    /// Replay all events through projections, rebuilding read models.
    #[cfg(test)]
    #[deprecated]
    pub(crate) fn replay(&self) -> Result<usize, EventError> {
        let projections = Projections::<BrainCanon>::project();
        projections.replay_brain(&self.db()?)
    }

    /// Emit an event through the bus.
    ///
    /// The bus appends to the event log, dispatches to the consumer
    /// for projection and chronicling, and broadcasts to SSE
    /// subscribers. Returns after projection is complete.
    pub(crate) async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let bus = self
            .bus
            .as_ref()
            .ok_or_else(|| EventError::Lock("emit requires a running bus".to_string()))?;

        bus.emit(event, Source::default()).await?;

        Ok(())
    }

    /// Execute a read operation against the bus's database.
    ///
    /// Uses the bus's shared connection when available, falls back to
    /// opening a fresh connection for client-only contexts.
    pub(crate) fn with_db<T>(
        &self,
        f: impl FnOnce(&rusqlite::Connection) -> T,
    ) -> Result<T, EventError> {
        match &self.bus {
            Some(bus) => bus.with_db(f),
            None => {
                let db = self.db()?;
                Ok(f(&db))
            }
        }
    }
}
