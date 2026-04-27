use std::sync::Arc;

use crate::*;

impl aide::operation::OperationInput for SystemContext {}

#[derive(Clone)]
pub struct SystemContext {
    pub config: Config,
    pub projections: Projections<SystemCanon>,
    stream: SystemStream,
}

impl SystemContext {
    pub fn new(config: Config) -> Self {
        let projections = Projections::system();
        let subscribers: Vec<Arc<dyn Subscriber>> = vec![Arc::new(
            SystemProjectionSubscriber::new(projections.clone(), config.clone()),
        )];
        let stream = SystemStream::new(config.clone(), subscribers);
        Self {
            config,
            projections,
            stream,
        }
    }

    /// Build an HTTP client for system operations.
    pub fn client(&self) -> Client {
        Client::new(self.config.base_url())
    }

    /// Open the system database.
    pub fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.system_db()
    }

    /// Emit an event to the system event log and apply projections.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        self.stream.publish(event)?;
        Ok(())
    }

    /// Ceremony barrier — wait until subscribers have caught up to the
    /// current log head. Call before reading projected state after a
    /// multi-emit workflow.
    pub async fn wait_for_head(&self) -> Result<(), EventError> {
        self.stream.wait_for_head().await
    }
}
