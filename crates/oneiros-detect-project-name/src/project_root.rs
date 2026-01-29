use std::path::PathBuf;

/// A detected project root with its name and filesystem path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRoot {
    /// The project name (e.g., from `Cargo.toml`, directory name, etc.)
    pub name: String,
    /// The absolute path to the project root directory.
    pub path: PathBuf,
}

impl ProjectRoot {
    /// Create a new `ProjectRoot`.
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}
