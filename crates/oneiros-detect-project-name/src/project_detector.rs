use std::path::Path;

use crate::{
    detection_strategy::DetectionStrategy, directory::Directory, git::Git, package::Package,
    project_root::ProjectRoot, workspace::Workspace,
};

/// Detects project roots using an ordered chain of strategies.
///
/// Strategies are evaluated in order. The first one that returns `Some` wins.
pub struct ProjectDetector {
    strategies: Vec<Box<dyn DetectionStrategy>>,
}

impl ProjectDetector {
    /// Create a detector with the default strategy chain:
    /// 1. Workspace (Cargo.toml with [workspace])
    /// 2. Package (topmost Cargo.toml with [package])
    /// 3. Git (.git/ directory)
    /// 4. Directory (current directory name)
    pub fn default_chain() -> Self {
        Self::with_strategies(vec![
            Box::new(Workspace),
            Box::new(Package),
            Box::new(Git),
            Box::new(Directory),
        ])
    }

    /// Create a detector with a custom strategy chain.
    pub(crate) fn with_strategies(strategies: Vec<Box<dyn DetectionStrategy>>) -> Self {
        Self { strategies }
    }

    /// Detect a project root starting from the given path.
    ///
    /// Tries each strategy in order, returning the first successful detection.
    /// Returns `None` if no strategy matches (only possible with custom chains
    /// that exclude the `Directory` fallback).
    pub fn detect(&self, start: &Path) -> Option<ProjectRoot> {
        self.strategies.iter().find_map(|s| s.detect(start))
    }
}

impl Default for ProjectDetector {
    fn default() -> Self {
        Self::default_chain()
    }
}

#[cfg(test)]
mod testsnu {
    use super::*;

    #[test]
    fn workspace_wins_over_git() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
    [workspace]
    members = []

    [workspace.package]
    name = "workspace-name"
    "#,
        )
        .unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();

        let root = ProjectDetector::default_chain().detect(dir.path()).unwrap();

        assert_eq!(root.name, "workspace-name");
    }

    #[test]
    fn package_wins_over_git() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
    [package]
    name = "package-name"
    version = "0.1.0"
    "#,
        )
        .unwrap();

        std::fs::create_dir(dir.path().join(".git")).unwrap();

        assert_eq!(
            ProjectDetector::default_chain()
                .detect(dir.path())
                .unwrap()
                .name,
            "package-name"
        );
    }

    #[test]
    fn git_wins_over_directory() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let nested = dir.path().join("some-subdir");

        std::fs::create_dir(dir.path().join(".git")).unwrap();
        std::fs::create_dir(&nested).unwrap();

        assert_eq!(
            ProjectDetector::default_chain()
                .detect(&nested)
                .unwrap()
                .path,
            dir.path().canonicalize().unwrap()
        );
    }

    #[test]
    fn falls_back_to_directory() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let root = ProjectDetector::default_chain().detect(dir.path()).unwrap();

        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn custom_chain_without_fallback_can_return_none() {
        assert!(
            ProjectDetector::with_strategies(vec![Box::new(Workspace)])
                .detect(
                    tempfile::tempdir()
                        .expect("failed to create temp dir")
                        .path()
                )
                .is_none()
        );
    }
}
