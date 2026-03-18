use serde::{Deserialize, Serialize};

use super::model::StorageEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    BlobStored(StorageEntry),
    BlobRemoved(BlobRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRemoved {
    pub id: String,
}
