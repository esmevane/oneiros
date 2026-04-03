use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageResponse {
    StorageSet(Response<StorageEntry>),
    StorageDetails(Response<StorageEntry>),
    Entries(Listed<Response<StorageEntry>>),
    NoEntries,
    StorageRemoved(StorageKey),
}
