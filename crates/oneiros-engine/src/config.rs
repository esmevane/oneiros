//! Engine configuration — pure data, loaded at startup.

use std::path::PathBuf;

/// Configuration for the engine.
///
/// Carries paths and tuning knobs. Passed to contexts that need
/// filesystem access (storage, export/import).
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    pub data_dir: PathBuf,
}

impl Config {
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }
}
