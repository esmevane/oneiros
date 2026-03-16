use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SelectStorageByKey {
    pub key: StorageKey,
}

// ── Request types ──────────────────────────────────────────────────

/// Storage set request. The `data` field carries raw bytes and is excluded
/// from JSON serialization — each transport is responsible for encoding
/// binary content appropriately (raw body for HTTP, base64 for MCP, etc.)
#[derive(Debug, Clone)]
pub struct SetStorageRequest {
    pub key: StorageKey,
    pub description: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetStorageRequest {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetStorageContentRequest {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RemoveStorageRequest {
    pub key: StorageKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListStorageRequest;

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    BlobStored(BlobContent),
    StorageSet(StorageEntry),
    StorageRemoved(SelectStorageByKey),
}

// Note: StorageRequests does not include SetStorage or GetStorageContent
// because both carry binary data that doesn't fit the JSON {type, data}
// envelope. They are dispatched directly via OneirosService bypass methods.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageRequests {
    RemoveStorage(RemoveStorageRequest),
    GetStorage(GetStorageRequest),
    ListStorage(ListStorageRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageResponses {
    StorageSet(StorageEntry),
    StorageFound(StorageEntry),
    StorageListed(Vec<StorageEntry>),
    StorageRemoved,
}
