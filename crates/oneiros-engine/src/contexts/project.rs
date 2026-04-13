use tokio::sync::broadcast;

use crate::*;

#[derive(Clone)]
pub(crate) struct ProjectContext {
    pub(crate) config: Config,
    pub(crate) projections: Projections<BrainCanon>,
    chronicle: Chronicle,
    broadcast: broadcast::Sender<StoredEvent>,
}

impl ProjectContext {
    pub(crate) fn new(config: Config) -> Self {
        let (broadcast, _) = broadcast::channel(256);

        Self {
            config,
            projections: Projections::project(),
            chronicle: Chronicle::new(),
            broadcast,
        }
    }

    /// Create a context with shared broadcast and a pre-hydrated bookmark entry.
    pub(crate) fn with_entry(
        config: Config,
        broadcast: broadcast::Sender<StoredEvent>,
        entry: BookmarkEntry,
    ) -> Self {
        Self {
            config,
            projections: Projections::project_with_entry(entry.canon, entry.pipeline),
            chronicle: entry.chronicle,
            broadcast,
        }
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
        self.broadcast.subscribe()
    }

    /// Open the brain (project) database.
    pub(crate) fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.brain_db()
    }

    /// Replay all events through projections, rebuilding read models.
    #[cfg(test)]
    #[deprecated]
    pub(crate) fn replay(&self) -> Result<usize, EventError> {
        self.projections.replay_brain(&self.db()?)
    }

    /// Emit an event to the brain's event log and apply projections.
    pub(crate) async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::new(&db).append(&new_event)?;

        self.projections.apply_brain(&db, &stored)?;

        // Chronicle the event — record it in the active bookmark's ledger.
        let chronicle_store = ChronicleStore::new(&db);
        chronicle_store.migrate()?;
        self.chronicle.record(
            &stored,
            &chronicle_store.resolver(),
            &chronicle_store.writer(),
        )?;

        // We broadcast the new event after projecting it.
        let _ = self.broadcast.send(stored);

        Ok(())
    }
}
