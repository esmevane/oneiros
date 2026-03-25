use std::path::PathBuf;

/// A detected project root with its name and filesystem path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRoot {
    pub name: String,
    pub path: PathBuf,
}

impl ProjectRoot {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}
