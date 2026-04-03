use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageResponse {
    StorageSet(StorageEntry),
    StorageDetails(StorageEntry),
    Entries(Listed<StorageEntry>),
    NoEntries,
    StorageRemoved(StorageKey),
}
