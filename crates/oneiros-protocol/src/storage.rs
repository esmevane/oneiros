use oneiros_model::{StorageEntry, StorageKey};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntry),
    StorageRemoved { key: StorageKey },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageRequests {
    SetStorage(StorageEntry),
    RemoveStorage { key: StorageKey },
}
