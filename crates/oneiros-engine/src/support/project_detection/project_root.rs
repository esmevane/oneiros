use std::path::PathBuf;

/// A detected project root with its name and filesystem path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectRoot {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl ProjectRoot {
    pub(crate) fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}
