use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectStorageByKey {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntry),
    StorageRemoved(SelectStorageByKey),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageRequests {
    SetStorage(StorageEntry),
    RemoveStorage(SelectStorageByKey),
    GetStorage(SelectStorageByKey),
    ListStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageResponses {
    StorageSet(StorageEntry),
    StorageFound(StorageEntry),
    StorageListed(Vec<StorageEntry>),
    StorageRemoved,
}
