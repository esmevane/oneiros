use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    /// Persistent: projects to storage metadata table (upsert).
    StorageSet(StorageEntry),
    /// Persistent: removes storage metadata by key.
    StorageRemoved(SelectStorageByKey),
}
