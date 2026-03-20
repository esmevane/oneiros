use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    BlobStored(StorageEntry),
    BlobRemoved(BlobRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRemoved {
    pub id: StorageId,
}
