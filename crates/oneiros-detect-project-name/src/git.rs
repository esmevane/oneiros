use std::path::Path;

use crate::{detection_strategy::DetectionStrategy, project_root::ProjectRoot};

/// Detects Git repository roots.
///
/// Walks up the directory tree looking for a `.git/` directory.
/// The project name is taken from the directory containing `.git/`.
pub(crate) struct Git;

impl DetectionStrategy for Git {
    fn detect(&self, start: &Path) -> Option<ProjectRoot> {
        let mut current = start.canonicalize().ok()?;

        loop {
            let git_dir = current.join(".git");

            if git_dir.exists() {
                let name = current
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(String::from)?;

                return Some(ProjectRoot::new(name, &current));
            }

            if !current.pop() {
                break;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn detects_git_repo() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        fs::create_dir(dir.path().join(".git")).unwrap();

        let root = Git.detect(dir.path()).unwrap();

        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn walks_up_to_find_git() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let nested = dir.path().join("src").join("deep");

        fs::create_dir(dir.path().join(".git")).unwrap();
        fs::create_dir_all(&nested).unwrap();

        let root = Git.detect(&nested).unwrap();

        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn returns_none_without_git() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let result = Git.detect(dir.path());

        assert!(result.is_none());
    }
}
