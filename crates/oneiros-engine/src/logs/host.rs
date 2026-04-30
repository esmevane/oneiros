use std::sync::Arc;

use crate::*;

impl aide::operation::OperationInput for HostLog {}

#[derive(Clone)]
pub struct HostLog {
    pub config: Config,
    pub projections: Projections<SystemCanon>,
    /// Lazily-composed Scope, cached for the context's lifetime.
    /// Wrapped in Arc so Clone is cheap.
    scope: Arc<std::sync::OnceLock<Scope<AtHost>>>,
}

impl HostLog {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            projections: Projections::system(),
            scope: Arc::new(std::sync::OnceLock::new()),
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

    /// Compose a host-tier Scope from this context's config (lazy,
    /// cached). Strangler helper: lets services pass &Scope to repos
    /// without changing their own signatures during migration.
    pub fn scope(&self) -> Result<&Scope<AtHost>, ComposeError> {
        if self.scope.get().is_none() {
            let s = ComposeScope::new(self.config.clone()).host()?;
            let _ = self.scope.set(s);
        }
        Ok(self.scope.get().expect("just set above"))
    }

    /// Emit an event to the system event log and apply projections.
    pub async fn emit(&self, event: impl Into<Events>) -> Result<(), EventError> {
        let db = self.db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::new(&db).append(&new_event)?;

        self.projections.apply(&db, &stored)?;

        Ok(())
    }
}
