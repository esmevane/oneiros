use std::path::Path;

use crate::project_root::ProjectRoot;

/// A strategy for detecting a project root and name from a filesystem path.
///
/// Strategies are evaluated in order by [`ProjectDetector`](crate::ProjectDetector).
/// The first strategy that returns `Some` wins.
pub(crate) trait DetectionStrategy: Send + Sync {
    /// Attempt to detect a project root starting from the given path.
    ///
    /// Implementations typically walk up the directory tree looking for
    /// marker files or directories (e.g., `Cargo.toml`, `.git/`).
    ///
    /// Returns `Some(ProjectRoot)` if detection succeeds, `None` otherwise.
    fn detect(&self, start: &Path) -> Option<ProjectRoot>;
}
