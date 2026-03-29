use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::*;

/// The result of a successful project initialization — carries the
/// token needed for all subsequent authenticated requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitResult {
    pub brain_name: BrainName,
    pub token: Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectResponse {
    Initialized(InitResult),
    BrainAlreadyExists(BrainName),
    WroteExport(PathBuf),
    Imported(ImportResult),
    Replayed(ReplayResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported: usize,
    pub replayed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub replayed: usize,
}
