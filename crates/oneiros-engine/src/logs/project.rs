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
pub(crate) struct ProjectLog {
    pub(crate) config: Config,
    /// Lazily-composed Scope, cached for the context's lifetime.
    scope: Arc<std::sync::OnceLock<Scope<AtBookmark>>>,
}

impl ProjectLog {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            scope: Arc::new(std::sync::OnceLock::new()),
        }
    }

    /// The brain name for this project.
    pub(crate) fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// Compose a bookmark-tier Scope from this context's config
    /// (lazy, cached). Strangler helper: MCP dispatchers extract this
    /// to pass `&Scope<AtBookmark>` to migrated services.
    pub(crate) fn scope(&self) -> Result<&Scope<AtBookmark>, ComposeError> {
        if self.scope.get().is_none() {
            let s = ComposeScope::new(self.config.clone())
                .bookmark(self.config.brain.clone(), self.config.bookmark.clone())?;
            let _ = self.scope.set(s);
        }
        Ok(self.scope.get().expect("just set above"))
    }
}
