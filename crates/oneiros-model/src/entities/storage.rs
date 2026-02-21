use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type StorageEntryRecord = HasDescription<HasHash<StorageEntry>>;

impl StorageEntryRecord {
    pub fn init(
        description: impl Into<Description>,
        hash: impl Into<ContentHash>,
        entry: StorageEntry,
    ) -> Self {
        HasDescription::new(description.into(), HasHash::new(hash, entry))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StorageEntry {
    pub key: StorageKey,
}

impl StorageEntry {
    pub fn construct_from_db(
        (key, description, hash): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> StorageEntryRecord {
        StorageEntryRecord::init(
            Description::new(description),
            ContentHash::new(hash),
            StorageEntry {
                key: StorageKey::new(key),
            },
        )
    }
}

impl<GivenKey> From<GivenKey> for StorageEntry
where
    GivenKey: AsRef<str>,
{
    fn from(key: GivenKey) -> Self {
        StorageEntry {
            key: StorageKey::new(key),
        }
    }
}

domain_link!(StorageEntry => StorageEntryLink);
domain_name!(StorageKey);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_identity() {
        let primary = StorageEntry {
            key: StorageKey::new("config.toml"),
        };

        let other = StorageEntry {
            key: StorageKey::new("config.toml"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
