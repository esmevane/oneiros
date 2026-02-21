use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub description: Description,
    pub hash: ContentHash,
}

impl StorageEntry {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenKey, GivenDescription, GivenHash> From<(GivenKey, GivenDescription, GivenHash)>
    for StorageEntry
where
    GivenKey: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenHash: AsRef<str>,
{
    fn from((key, description, hash): (GivenKey, GivenDescription, GivenHash)) -> Self {
        StorageEntry {
            key: StorageKey::new(key),
            description: Description::new(description),
            hash: ContentHash::new(hash),
        }
    }
}

impl Addressable for StorageEntry {
    fn address_label() -> &'static str {
        "storage"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.key))
    }
}

domain_name!(StorageKey);
oneiros_link::domain_link!(StorageLink, "storage");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_identity() {
        let primary = StorageEntry {
            key: StorageKey::new("config.toml"),
            description: Description::new("first"),
            hash: ContentHash::new("abc123"),
        };

        let other = StorageEntry {
            key: StorageKey::new("config.toml"),
            description: Description::new("updated"),
            hash: ContentHash::new("def456"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
