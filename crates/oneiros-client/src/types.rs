use std::path::PathBuf;

use oneiros_model::{BrainStatus, Id, Label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrainRequest {
    pub name: Label,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub id: Id,
    pub name: Label,
    pub path: PathBuf,
    pub status: BrainStatus,
}
