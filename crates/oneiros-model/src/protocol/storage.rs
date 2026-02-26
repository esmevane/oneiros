use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntry),
    StorageRemoved { key: StorageKey },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageRequests {
    SetStorage(StorageEntry),
    RemoveStorage { key: StorageKey },
    GetStorage { key: StorageKey },
    ListStorage,
}
