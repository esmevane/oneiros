use std::path::PathBuf;

use serde::Serialize;

use crate::*;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectResponse {
    BrainCreated(BrainName),
    BrainAlreadyExists(BrainName),
    WroteExport(PathBuf),
    Imported(ImportResult),
    Replayed(ReplayResult),
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub replayed: usize,
}

#[derive(Debug, Serialize)]
pub struct ReplayResult {
    pub replayed: usize,
}
