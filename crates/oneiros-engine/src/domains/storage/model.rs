use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

/// Storage metadata entry — maps a human-readable key to a content-addressed blob.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum StorageEntry {
    Current(StorageEntryV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct StorageEntryV1 {
    #[builder(into)]
    pub key: StorageKey,
    #[builder(into)]
    pub description: Description,
    pub hash: ContentHash,
}

impl StorageEntry {
    pub fn build_v1() -> StorageEntryV1Builder {
        StorageEntryV1::builder()
    }

    pub fn key(&self) -> &StorageKey {
        match self {
            Self::Current(v) => &v.key,
        }
    }

    pub fn description(&self) -> &Description {
        match self {
            Self::Current(v) => &v.description,
        }
    }

    pub fn hash(&self) -> &ContentHash {
        match self {
            Self::Current(v) => &v.hash,
        }
    }
}

#[derive(Clone, Default)]
pub struct StorageEntries(HashMap<String, StorageEntry>);

impl StorageEntries {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, key: &StorageKey) -> Option<&StorageEntry> {
        self.0.get(&key.to_string())
    }

    pub fn set(&mut self, entry: &StorageEntry) -> Option<StorageEntry> {
        self.0.insert(entry.key().to_string(), entry.clone())
    }

    pub fn remove(&mut self, key: &StorageKey) -> Option<StorageEntry> {
        self.0.remove(&key.to_string())
    }
}

/// Binary content for transport — carries compressed blob data in the event stream.
///
/// Used by the transient `BlobStored` event: the projection materializes the blob
/// into the `blob` table, then deletes the event to keep the log lean.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum BlobContent {
    Current(BlobContentV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema)]
pub struct BlobContentV1 {
    pub hash: ContentHash,
    pub size: Size,
    pub data: Blob,
}

impl BlobContent {
    pub fn build_v1() -> BlobContentV1Builder {
        BlobContentV1::builder()
    }

    /// Create a new BlobContent from raw bytes — computes hash, compresses data.
    pub fn create(data: &[u8]) -> Result<Self, BlobError> {
        let hash = ContentHash::compute(data);
        let size = Size::new(data.len());
        let blob = Blob::compressed(data)?;

        Ok(Self::Current(BlobContentV1 {
            hash,
            size,
            data: blob,
        }))
    }

    pub fn hash(&self) -> &ContentHash {
        match self {
            Self::Current(v) => &v.hash,
        }
    }

    pub fn size(&self) -> Size {
        match self {
            Self::Current(v) => v.size,
        }
    }

    pub fn data(&self) -> &Blob {
        match self {
            Self::Current(v) => &v.data,
        }
    }
}

/// Selector for storage removal by key.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SelectStorageByKey {
    Current(SelectStorageByKeyV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema)]
pub struct SelectStorageByKeyV1 {
    pub key: StorageKey,
}

impl SelectStorageByKey {
    pub fn build_v1() -> SelectStorageByKeyV1Builder {
        SelectStorageByKeyV1::builder()
    }

    pub fn key(&self) -> &StorageKey {
        match self {
            Self::Current(v) => &v.key,
        }
    }
}

resource_name!(StorageKey);
