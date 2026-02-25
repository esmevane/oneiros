use oneiros_link::*;
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

domain_link!(StorageEntry => StorageEntryLink);
domain_name!(StorageKey);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_same_fields_same_link() {
        let primary = StorageEntry::init(
            StorageKey::new("config.toml"),
            "A config file",
            ContentHash::new("abc123"),
        );

        let other = StorageEntry::init(
            StorageKey::new("config.toml"),
            "A config file",
            ContentHash::new("abc123"),
        );

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }

    #[test]
    fn storage_different_hash_different_link() {
        let primary = StorageEntry::init(
            StorageKey::new("config.toml"),
            "A config file",
            ContentHash::new("abc123"),
        );

        let other = StorageEntry::init(
            StorageKey::new("config.toml"),
            "A config file",
            ContentHash::new("different-hash"),
        );

        assert_ne!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
