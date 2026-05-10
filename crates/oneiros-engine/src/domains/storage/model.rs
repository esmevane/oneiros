use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// Storage metadata entry — maps a human-readable key to a content-addressed blob.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct StorageEntry {
    pub(crate) key: StorageKey,
    pub(crate) description: Description,
    pub(crate) hash: ContentHash,
}

impl Indexable<StorageKey> for StorageEntry {
    fn id(&self) -> StorageKey {
        self.key.clone()
    }
}

pub(crate) type StorageEntries = EntityIndex<StorageKey, StorageEntry>;

/// Binary content for transport — carries compressed blob data in the event stream.
///
/// Used by the transient `BlobStored` event: the projection materializes the blob
/// into the `blob` table, then deletes the event to keep the log lean.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct BlobContent {
    pub(crate) hash: ContentHash,
    pub(crate) size: Size,
    pub(crate) data: Blob,
}

impl BlobContent {
    /// Create a new BlobContent from raw bytes — computes hash, compresses data.
    pub(crate) fn create(data: &[u8]) -> Result<Self, BlobError> {
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

resource_name!(StorageKey);
