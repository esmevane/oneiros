use tokio::sync::broadcast;

use crate::*;

#[derive(Clone)]
pub struct ProjectContext {
    pub config: Config,
    pub projections: Projections,
    broadcast: broadcast::Sender<StoredEvent>,
}

impl ProjectContext {
    pub fn new(config: Config) -> Self {
        let (broadcast, _) = broadcast::channel(256);

        Self {
            config,
            projections: Projections::project(),
            broadcast,
        }
    }

    /// Create a context that shares an existing broadcast channel.
    ///
    /// Used by the HTTP server so all per-request contexts and SSE
    /// subscribers share the same event stream.
    pub fn with_broadcast(config: Config, broadcast: broadcast::Sender<StoredEvent>) -> Self {
        Self {
            config,
            projections: Projections::project(),
            broadcast,
        }
    }

    /// The brain name for this project.
    pub fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// Build an authenticated HTTP client for this project.
    ///
    /// Reads the token from the token file on disk. Returns an
    /// unauthenticated client if no token file exists yet (e.g. before
    /// project init).
    pub fn client(&self) -> Client {
        match self.config.token() {
            Some(token) => Client::with_token(self.config.base_url(), token),
            None => Client::new(self.config.base_url()),
        }
    }

    /// Subscribe to the event broadcast stream.
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.broadcast.subscribe()
    }

    /// Open the brain (project) database.
    pub fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.brain_db()
    }

    /// Open the system database.
    pub fn system_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.system_db()
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(&self) -> Result<usize, EventError> {
        self.projections.replay(&self.db()?)
    }

    /// Emit an event to the brain's event log and apply projections.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::new(&db).append(&new_event)?;

        self.projections.apply(&db, &stored)?;

        // We broadcast the new event after projecting it.
        let _ = self.broadcast.send(stored);

        Ok(())
    }
}
