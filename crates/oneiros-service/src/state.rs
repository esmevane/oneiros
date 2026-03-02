use std::path::PathBuf;
use std::sync::Mutex;

use oneiros_db::Database;
use oneiros_model::Events;
use tokio::sync::broadcast;

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
}
