use std::path::Path;

use crate::{detection_strategy::DetectionStrategy, project_root::ProjectRoot};

/// Detects Cargo package roots.
///
/// Walks up the directory tree looking for `Cargo.toml` files with a
/// `[package]` section. Takes the *topmost* package found (keeps walking
/// up even after finding one). The project name is taken from `package.name`.
pub(crate) struct Package;

impl DetectionStrategy for Package {
    fn detect(&self, start: &Path) -> Option<ProjectRoot> {
        let mut current = start.canonicalize().ok()?;
        let mut topmost: Option<ProjectRoot> = None;

        loop {
            let cargo_toml = current.join("Cargo.toml");

            if cargo_toml.exists() {
                let maybe_package = {
                    let contents = std::fs::read_to_string(cargo_toml).ok()?;
                    let parsed: toml::Table = contents.parse().ok()?;

                    let package = parsed.get("package")?.as_table()?;
                    let name = package.get("name")?.as_str()?;

                    Some(ProjectRoot::new(name, &current))
                };

                if let Some(root) = maybe_package {
                    topmost = Some(root);
                }
            }

            if !current.pop() {
                break;
            }
        }

        topmost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_package() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let cargo_toml = dir.path().join("Cargo.toml");

        std::fs::write(
            &cargo_toml,
            r#"
    [package]
    name = "my-package"
    version = "0.1.0"
    "#,
        )
        .unwrap();

        let root = Package.detect(dir.path()).unwrap();

        assert_eq!(root.name, "my-package");
    }

    #[test]
    fn finds_topmost_package() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let nested = dir.path().join("crates").join("nested");

        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
    [package]
    name = "root-package"
    version = "0.1.0"
    "#,
        )
        .unwrap();

        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(
            nested.join("Cargo.toml"),
            r#"
    [package]
    name = "nested-package"
    version = "0.1.0"
    "#,
        )
        .unwrap();

        let root = Package.detect(&nested).unwrap();

        assert_eq!(root.name, "root-package");
        assert_eq!(root.path, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn ignores_workspace_only_cargo_toml() {
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

        let result = Package.detect(dir.path());

        assert!(result.is_none());
    }
}
