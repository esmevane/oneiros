use std::path::Path;

use super::ProjectRoot;

/// A strategy for detecting a project root and name from a filesystem path.
///
/// Strategies are evaluated in order by [`ProjectDetector`](super::ProjectDetector).
/// The first strategy that returns `Some` wins.
pub(crate) trait DetectionStrategy: Send + Sync {
    fn detect(&self, start: &Path) -> Option<ProjectRoot>;
}
