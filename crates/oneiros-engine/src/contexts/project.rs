use std::sync::Arc;

use tokio::sync::broadcast;

use crate::*;

impl aide::operation::OperationInput for ProjectContext {}

#[derive(Clone)]
pub struct ProjectContext {
    pub config: Config,
    pub projections: Projections<BrainCanon>,
    stream: ProjectStream,
}

impl ProjectContext {
    pub fn new(config: Config) -> Self {
        let (wake, _) = broadcast::channel(256);
        let projections = Projections::project();
        let chronicle = Chronicle::new();
        Self::assemble(config, wake, projections, chronicle)
    }

    /// Create a context that shares an existing broadcast channel.
    ///
    /// Used by the HTTP server so all per-request contexts and SSE
    /// subscribers share the same event stream.
    pub fn with_broadcast(config: Config, wake: broadcast::Sender<StoredEvent>) -> Self {
        let projections = Projections::project();
        let chronicle = Chronicle::new();
        Self::assemble(config, wake, projections, chronicle)
    }

    /// Create a context with shared broadcast and a pre-hydrated bookmark entry.
    pub fn with_entry(
        config: Config,
        wake: broadcast::Sender<StoredEvent>,
        entry: BookmarkEntry,
    ) -> Self {
        let projections = Projections::project_with_pipeline(entry.pipeline);
        Self::assemble(config, wake, projections, entry.chronicle)
    }

    /// Shared constructor — wires up the stream with its subscribers.
    fn assemble(
        config: Config,
        wake: broadcast::Sender<StoredEvent>,
        projections: Projections<BrainCanon>,
        chronicle: Chronicle,
    ) -> Self {
        let subscribers: Vec<Arc<dyn Subscriber>> = vec![
            Arc::new(ProjectionSubscriber::new(
                projections.clone(),
                config.clone(),
            )),
            Arc::new(ChronicleSubscriber::new(chronicle, config.clone())),
        ];
        let stream = ProjectStream::new(config.clone(), wake, subscribers);
        Self {
            config,
            projections,
            stream,
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
        self.stream.subscribe()
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
        self.stream.publish(event)?;
        Ok(())
    }

    /// Ceremony barrier — wait until subscribers (projections, chronicle)
    /// have caught up to the current log head. Call before reading back
    /// projected state in multi-emit workflows.
    pub async fn wait_for_head(&self) -> Result<(), EventError> {
        self.stream.wait_for_head().await
    }
}
