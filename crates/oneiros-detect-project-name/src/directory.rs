use std::path::Path;

use crate::{detection_strategy::DetectionStrategy, project_root::ProjectRoot};

/// Falls back to using the current directory name.
///
/// Does not walk up the directory tree. Simply uses the starting directory's
/// name as the project name. This is the last-resort fallback in the default
/// detection chain.
pub(crate) struct Directory;

impl DetectionStrategy for Directory {
    fn detect(&self, start: &Path) -> Option<ProjectRoot> {
        let canonical = start.canonicalize().ok()?;
        let name = canonical
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)?;

        Some(ProjectRoot::new(name, canonical))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uses_directory_name() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let root = Directory.detect(dir.path()).unwrap();

        let expected_name = dir
            .path()
            .canonicalize()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        assert_eq!(root.name, expected_name);
        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn does_not_walk_up() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let nested = dir.path().join("nested");

        std::fs::create_dir(&nested).unwrap();

        let root = Directory.detect(&nested).unwrap();

        assert_eq!(root.path, nested.canonicalize().unwrap());
    }
}
