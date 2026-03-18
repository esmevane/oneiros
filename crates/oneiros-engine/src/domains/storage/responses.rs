use serde::{Deserialize, Serialize};

use super::model::StorageEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageResponse {
    Uploaded(StorageEntry),
    Found(StorageEntry),
    Listed(Vec<StorageEntry>),
    Removed,
}
