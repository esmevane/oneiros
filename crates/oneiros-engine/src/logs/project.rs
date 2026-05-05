use std::sync::Arc;

use crate::*;

impl aide::operation::OperationInput for ProjectLog {}

/// Strangler — request-shaped context for legacy bookmark-tier callers.
///
/// Carries the request's `Config` (with brain + bookmark already
/// resolved) and provides a lazily-composed `Scope<AtBookmark>` plus
/// an authenticated HTTP `client()`. New code uses
/// `Scope<AtBookmark>` + `Mailbox` directly; this remains for CLI
/// commands and MCP dispatchers that derive their scope from the
/// request context during the bus migration.
#[derive(Clone)]
pub struct ProjectLog {
    pub config: Config,
    /// Lazily-composed Scope, cached for the context's lifetime.
    scope: Arc<std::sync::OnceLock<Scope<AtBookmark>>>,
}

impl ProjectLog {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            scope: Arc::new(std::sync::OnceLock::new()),
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

    /// Compose a bookmark-tier Scope from this context's config
    /// (lazy, cached). Strangler helper: MCP dispatchers extract this
    /// to pass `&Scope<AtBookmark>` to migrated services.
    pub fn scope(&self) -> Result<&Scope<AtBookmark>, ComposeError> {
        if self.scope.get().is_none() {
            let s = ComposeScope::new(self.config.clone())
                .bookmark(self.config.brain.clone(), self.config.bookmark.clone())?;
            let _ = self.scope.set(s);
        }
        Ok(self.scope.get().expect("just set above"))
    }
}
