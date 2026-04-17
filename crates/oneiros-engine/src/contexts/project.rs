use tokio::sync::broadcast;

use crate::*;

impl aide::operation::OperationInput for ProjectContext {}

#[derive(Clone)]
pub struct ProjectContext {
    pub config: Config,
    pub projections: Projections<BrainCanon>,
    chronicle: Chronicle,
    broadcast: broadcast::Sender<StoredEvent>,
}

impl ProjectContext {
    pub fn new(config: Config) -> Self {
        let (broadcast, _) = broadcast::channel(256);

        Self {
            config,
            projections: Projections::project(),
            chronicle: Chronicle::new(),
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
            chronicle: Chronicle::new(),
            broadcast,
        }
    }

    /// Create a context with shared broadcast and a pre-hydrated bookmark entry.
    pub fn with_entry(
        config: Config,
        broadcast: broadcast::Sender<StoredEvent>,
        entry: BookmarkEntry,
    ) -> Self {
        Self {
            config,
            projections: Projections::project_with_pipeline(entry.pipeline),
            chronicle: entry.chronicle,
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
            Some(token) => Client::with_token(self.config.base_url(), token)
                .unwrap_or_else(|_| Client::new(self.config.base_url())),
            None => Client::new(self.config.base_url()),
        }
    }

    /// Subscribe to the event broadcast stream.
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.broadcast.subscribe()
    }

    /// Open the bookmark DB with the events DB ATTACHed.
    ///
    /// Unqualified table names resolve to the bookmark DB (projections).
    /// Event log operations use the `events` schema qualifier via
    /// `EventLog::attached`.
    pub fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.bookmark_conn()
    }

    /// Open the system database.
    pub fn system_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.system_db()
    }

    /// Replay all events through projections, rebuilding read models.
    pub fn replay(&self) -> Result<usize, EventError> {
        let db = self.db()?;
        let log = EventLog::attached(&db);
        self.projections.replay_brain(&db, &log)
    }

    /// Emit an event to the brain's event log and apply projections.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::attached(&db).append(&new_event)?;

        self.projections.apply_brain(&db, &stored)?;

        // Chronicle the event in the system DB (shared across bookmarks).
        let system_db = self.system_db()?;
        let chronicle_store = ChronicleStore::new(&system_db);
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
