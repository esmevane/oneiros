use std::path::Path;

use crate::{detection_strategy::DetectionStrategy, project_root::ProjectRoot};

/// Detects Cargo workspace roots.
///
/// Walks up the directory tree looking for a `Cargo.toml` containing a
/// `[workspace]` section. The project name is taken from
/// `workspace.package.name` if present, otherwise the directory name.
pub(crate) struct Workspace;

impl DetectionStrategy for Workspace {
    fn detect(&self, start: &Path) -> Option<ProjectRoot> {
        let mut current = start.canonicalize().ok()?;

        loop {
            let cargo_toml = current.join("Cargo.toml");

            if cargo_toml.exists() {
                let maybe_workspace = {
                    let contents = std::fs::read_to_string(cargo_toml).ok()?;
                    let parsed: toml::Table = contents.parse().ok()?;

                    let workspace = parsed.get("workspace")?.as_table()?;

                    let name = workspace
                        .get("package")
                        .and_then(|p| p.as_table())
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .map(String::from)
                        .or_else(|| {
                            current
                                .file_name()
                                .and_then(|n| n.to_str())
                                .map(String::from)
                        })?;

                    Some(ProjectRoot::new(name, &current))
                };

                if let Some(root) = maybe_workspace {
                    return Some(root);
                }
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
    use super::*;

    #[test]
    fn detects_workspace_with_package_name() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let cargo_toml = dir.path().join("Cargo.toml");

        std::fs::write(
            &cargo_toml,
            r#"
    [workspace]
    members = ["crates/*"]

    [workspace.package]
    name = "my-workspace"
    "#,
        )
        .unwrap();

        let root = Workspace.detect(dir.path()).unwrap();

        assert_eq!(root.name, "my-workspace");
        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn detects_workspace_falls_back_to_dir_name() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let cargo_toml = dir.path().join("Cargo.toml");

        std::fs::write(
            &cargo_toml,
            r#"
    [workspace]
    members = ["crates/*"]
    "#,
        )
        .unwrap();

        let root = Workspace.detect(dir.path()).unwrap();

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
    }

    #[test]
    fn walks_up_to_find_workspace() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let nested = dir.path().join("crates").join("sub-crate");

        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
    [workspace]
    members = ["crates/*"]

    [workspace.package]
    name = "root-workspace"
    "#,
        )
        .unwrap();

        std::fs::create_dir_all(&nested).unwrap();

        let root = Workspace.detect(&nested).unwrap();

        assert_eq!(root.name, "root-workspace");
        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn ignores_package_only_cargo_toml() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let cargo_toml = dir.path().join("Cargo.toml");

        std::fs::write(
            &cargo_toml,
            r#"
    [package]
    name = "just-a-package"
    version = "0.1.0"
    "#,
        )
        .unwrap();

        let result = Workspace.detect(dir.path());

        assert!(result.is_none());
    }
}
