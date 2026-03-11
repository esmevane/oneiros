use std::path::PathBuf;
use std::sync::Mutex;

use oneiros_db::Database;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::*;

pub struct ServiceState {
    pub(crate) database: Mutex<Database>,
    pub(crate) data_dir: PathBuf,
    pub(crate) event_tx: broadcast::Sender<Event>,
    pub(crate) source: Source,
}

impl ServiceState {
    pub fn new(database: Database, data_dir: PathBuf, source: Source) -> Self {
        let (event_tx, _) = broadcast::channel(256);

        Self {
            database: Mutex::new(database),
            data_dir,
            event_tx,
            source,
        }
    }

    /// Subscribe to the event broadcast channel.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }

    /// Send an event to the broadcast channel (for testing).
    pub fn broadcast(&self, event: Event) {
        let _ = self.event_tx.send(event);
    }

    /// Acquire the system database lock.
    pub fn lock_database(&self) -> Result<std::sync::MutexGuard<'_, Database>, Error> {
        self.database.lock().map_err(|_| Error::DatabasePoisoned)
    }

    /// Access the event broadcast sender.
    pub fn event_sender(&self) -> &broadcast::Sender<Event> {
        &self.event_tx
    }

    /// Access the resolved system identity.
    pub fn source(&self) -> Source {
        self.source
    }

    /// Access the data directory path.
    pub fn data_dir(&self) -> &std::path::Path {
        &self.data_dir
    }

    /// Open a brain database and collect aggregate stats for the dashboard.
    pub fn brain_summary(&self, brain: &Brain) -> Result<BrainResponses, Error> {
        let db = Database::open_brain(&brain.path)?;

        let agents = db.list_agents()?;
        let cognitions = db.list_cognitions()?;
        let cognition_count = cognitions.len();
        let recent_cognitions = cognitions.into_iter().rev().take(30).collect();

        Ok(BrainResponses::BrainSummarized(BrainSummary {
            agents,
            cognition_count,
            memory_count: db.list_memories()?.len(),
            experience_count: db.list_experiences()?.len(),
            connection_count: db.list_connections()?.len(),
            event_count: db.event_count()?,
            recent_cognitions,
        }))
    }
}
