use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

/// Storage metadata entry — maps a human-readable key to a content-addressed blob.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub description: Description,
    pub hash: ContentHash,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "storage")]
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
        self.0.insert(entry.key.to_string(), entry.clone())
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
pub struct BlobContent {
    pub hash: ContentHash,
    pub size: Size,
    pub data: Blob,
}

impl BlobContent {
    /// Create a new BlobContent from raw bytes — computes hash, compresses data.
    pub fn create(data: &[u8]) -> Result<Self, BlobError> {
        let hash = ContentHash::compute(data);
        let size = Size::new(data.len());
        let blob = Blob::compressed(data)?;

        Ok(Self {
            hash,
            size,
            data: blob,
        })
    }
}

/// Selector for storage removal by key.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectStorageByKey {
    pub key: StorageKey,
}

resource_name!(StorageKey);
