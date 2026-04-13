use crate::*;

#[derive(Clone)]
pub(crate) struct SystemContext {
    pub(crate) config: Config,
    pub(crate) projections: Projections<SystemCanon>,
}

impl SystemContext {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            projections: Projections::system(),
        }
    }

    pub(crate) fn with_canon(config: Config, canon: Canon) -> Self {
        Self {
            config,
            projections: Projections::system_with_canon(canon),
        }
    }

    /// Build an HTTP client for system operations.
    pub(crate) fn client(&self) -> Client {
        Client::new(self.config.base_url())
    }

    /// Open the system database.
    pub(crate) fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.system_db()
    }

    /// Emit an event to the system event log and apply projections.
    pub(crate) async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::new(&db).append(&new_event)?;

        self.projections.apply(&db, &stored)?;

        Ok(())
    }
}
