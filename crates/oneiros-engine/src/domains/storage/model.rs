use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct StorageEntry {
    pub id: StorageId,
    #[serde(rename = "key")]
    pub name: StorageName,
    pub content_type: String,
    pub size: u64,
    pub created_at: String,
}

/// Binary content retrieved from storage.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StorageContent {
    pub entry: StorageEntry,
    pub data: Vec<u8>,
}

resource_id!(StorageId);
resource_name!(StorageName);
resource_name!(StorageKey);
