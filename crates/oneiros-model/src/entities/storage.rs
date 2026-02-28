use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub description: Description,
    pub hash: ContentHash,
}

impl StorageEntry {
    pub fn init(key: StorageKey, description: impl Into<Description>, hash: ContentHash) -> Self {
        Self {
            key,
            description: description.into(),
            hash,
        }
    }

    pub fn construct_from_db(
        (key, description, hash): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> StorageEntry {
        StorageEntry {
            key: StorageKey::new(key),
            description: Description::new(description),
            hash: ContentHash::new(hash),
        }
    }
}

impl<GivenKey> From<GivenKey> for StorageEntry
where
    GivenKey: AsRef<str>,
{
    fn from(key: GivenKey) -> Self {
        StorageEntry {
            key: StorageKey::new(key),
            description: Description::new(""),
            hash: ContentHash::new(""),
        }
    }
}

domain_name!(StorageKey);
