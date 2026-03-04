use std::path::PathBuf;
use std::sync::Mutex;

use oneiros_db::Database;
use oneiros_model::Events;
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

    /// Resolve a stored brain path to an absolute filesystem path.
    ///
    /// Old events stored absolute paths; new events store relative paths
    /// (`brains/{name}.db`). Absolute paths pass through unchanged; relative
    /// paths are resolved against `data_dir`.
    pub(crate) fn resolve_brain_path(&self, stored_path: impl AsRef<std::path::Path>) -> PathBuf {
        let path = stored_path.as_ref();
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.data_dir.join(path)
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

    /// Create a scoped service for system-level domain operations.
    ///
    /// Acquires the system database lock; the lock lives as long as the
    /// returned SystemService does.
    pub(crate) fn system_service(&self) -> Result<SystemService<'_>, Error> {
        let db = self.database.lock().map_err(|_| Error::DatabasePoisoned)?;
        Ok(SystemService::new(db, &self.data_dir, &self.event_tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_with_data_dir(data_dir: impl Into<PathBuf>) -> ServiceState {
        let db = Database::create(":memory:").unwrap();
        ServiceState::new(db, data_dir.into())
    }

    #[test]
    fn resolve_brain_path_passes_absolute_through() {
        let state = state_with_data_dir("/some/data");
        let resolved = state.resolve_brain_path("/absolute/path/brain.db");
        assert_eq!(resolved, PathBuf::from("/absolute/path/brain.db"));
    }

    #[test]
    fn resolve_brain_path_joins_relative_with_data_dir() {
        let state = state_with_data_dir("/some/data");
        let resolved = state.resolve_brain_path("brains/my-brain.db");
        assert_eq!(resolved, PathBuf::from("/some/data/brains/my-brain.db"));
    }
}
