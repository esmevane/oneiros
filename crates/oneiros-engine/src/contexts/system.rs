use crate::*;

#[derive(Clone)]
pub struct SystemContext {
    pub config: Config,
    pub projections: Projections,
}

impl SystemContext {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            projections: Projections::system(),
        }
    }

    /// Open the system database.
    pub fn db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        self.config.system_db()
    }

    /// Emit an event to the system event log and apply projections.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db().map_err(|e| EventError::Lock(e.to_string()))?;
        let new_event = NewEvent {
            data: event.into(),
            source: Source::default(),
        };
        let stored = EventLog::new(&db).append(&new_event)?;

        self.projections.apply(&db, &stored)?;

        Ok(())
    }
}
