use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub description: Description,
    pub hash: ContentHash,
}

impl<A, B, C> From<(A, B, C)> for StorageEntry
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
{
    fn from((key, description, hash): (A, B, C)) -> Self {
        StorageEntry {
            key: StorageKey::new(key),
            description: Description::new(description),
            hash: ContentHash::new(hash),
        }
    }
}

impl StorageEntry {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

domain_name!(StorageKey);
