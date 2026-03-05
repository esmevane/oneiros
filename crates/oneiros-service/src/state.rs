use std::path::PathBuf;
use std::sync::Mutex;

use oneiros_db::Database;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::Error;
use crate::system_service::SystemService;

pub struct ServiceState {
    pub(crate) database: Mutex<Database>,
    pub(crate) data_dir: PathBuf,
    pub(crate) event_tx: broadcast::Sender<Events>,
}

impl ServiceState {
    pub fn new(database: Database, data_dir: PathBuf) -> Self {
        let (event_tx, _) = broadcast::channel(256);

        Self {
            database: Mutex::new(database),
            data_dir,
            event_tx,
        }
    }

    /// Subscribe to the event broadcast channel.
    pub fn subscribe(&self) -> broadcast::Receiver<Events> {
        self.event_tx.subscribe()
    }

    /// Send an event to the broadcast channel (for testing).
    pub fn broadcast(&self, event: Events) {
        let _ = self.event_tx.send(event);
    }

    /// Acquire the system database lock.
    pub fn lock_database(&self) -> Result<std::sync::MutexGuard<'_, Database>, Error> {
        self.database.lock().map_err(|_| Error::DatabasePoisoned)
    }

    /// Access the event broadcast sender.
    pub fn event_sender(&self) -> &broadcast::Sender<Events> {
        &self.event_tx
    }

    /// Access the data directory path.
    pub fn data_dir(&self) -> &std::path::Path {
        &self.data_dir
    }

    /// Create a scoped service for system-level domain operations.
    ///
    /// Acquires the system database lock; the lock lives as long as the
    /// returned SystemService does.
    pub fn system_service(&self) -> Result<SystemService<'_>, Error> {
        let db = self.lock_database()?;
        Ok(SystemService::new(db, &self.data_dir, &self.event_tx))
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
