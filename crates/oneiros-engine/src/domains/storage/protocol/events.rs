use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = StorageEventsType, display = "kebab-case")]
pub enum StorageEvents {
    /// Persistent: projects to storage metadata table (upsert).
    StorageSet(StorageEntry),
    /// Persistent: removes storage metadata by key.
    StorageRemoved(SelectStorageByKey),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
