use std::path::PathBuf;

use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

/// The filesystem path where the export file was written.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ExportPath(pub(crate) PathBuf);

impl ExportPath {
    pub(crate) fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl AsRef<std::path::Path> for ExportPath {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

impl core::ops::Deref for ExportPath {
    type Target = PathBuf;
    fn deref(&self) -> &PathBuf {
        &self.0
    }
}

impl core::fmt::Display for ExportPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// A count of events processed (imported or replayed).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct EventCount(pub(crate) i64);

impl EventCount {
    pub(crate) fn new(value: impl Into<i64>) -> Self {
        Self(value.into())
    }
}

impl core::fmt::Display for EventCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// The result of a successful project initialization — carries the
/// token needed for all subsequent authenticated requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InitResult {
    pub(crate) brain_name: BrainName,
    pub(crate) token: Token,
}

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = ProjectResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ProjectResponse {
    Initialized(InitResult),
    BrainAlreadyExists(BrainName),
    WroteExport(ExportPath),
    Imported(ImportResult),
    Replayed(ReplayResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImportResult {
    pub(crate) imported: EventCount,
    pub(crate) replayed: EventCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReplayResult {
    pub(crate) replayed: EventCount,
}
