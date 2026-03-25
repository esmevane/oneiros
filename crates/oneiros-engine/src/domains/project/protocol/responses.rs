use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectResponse {
    BrainCreated(BrainName),
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
