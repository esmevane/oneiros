use crate::*;

impl aide::operation::OperationInput for HostLog {}

/// Strangler — request-shaped context for legacy host-tier callers.
///
/// Carries the request's `Config` and provides a `client()` for CLI
/// commands that dispatch through HTTP. New code uses
/// `Scope<AtHost>` + `Mailbox` directly; this exists to bridge CLI
/// commands and a few other holdouts during the bus migration.
#[derive(Clone)]
pub struct HostLog {
    pub config: Config,
}

impl HostLog {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build an HTTP client for system operations.
    pub fn client(&self) -> Client {
        Client::new(self.config.base_url())
    }
}
