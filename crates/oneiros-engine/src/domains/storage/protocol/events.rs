use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    /// Transient: carries binary content for export/import portability.
    /// The projection materializes the blob, then deletes this event.
    BlobStored(BlobContent),
    /// Persistent: projects to storage metadata table (upsert).
    StorageSet(StorageEntry),
    /// Persistent: removes storage metadata by key.
    StorageRemoved(SelectStorageByKey),
}
