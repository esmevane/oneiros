use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = StorageEventsType, display = "kebab-case")]
pub enum StorageEvents {
    /// Persistent: projects to storage metadata table (upsert).
    StorageSet(StorageSet),
    /// Persistent: removes storage metadata by key.
    StorageRemoved(StorageRemoved),
}

versioned! {
    pub enum StorageSet {
        V1 => {
            #[serde(flatten)] pub entry: StorageEntry,
        }
    }
}

versioned! {
    pub enum StorageRemoved {
        V1 => {
            pub key: StorageKey,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry() -> StorageEntry {
        StorageEntry {
            key: StorageKey::new("project-notes"),
            description: Description::new("notes"),
            hash: ContentHash::new("abc"),
        }
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (StorageEventsType::StorageSet, "storage-set"),
            (StorageEventsType::StorageRemoved, "storage-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn storage_set_wire_format_is_flat() {
        let event = StorageEvents::StorageSet(StorageSet::V1(StorageSetV1 {
            entry: sample_entry(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "storage-set");
        assert!(
            json["data"].get("entry").is_none(),
            "flatten must elide the entry envelope on the wire"
        );
        assert_eq!(json["data"]["key"], "project-notes");
        assert_eq!(json["data"]["description"], "notes");
        assert_eq!(json["data"]["hash"], "abc");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
