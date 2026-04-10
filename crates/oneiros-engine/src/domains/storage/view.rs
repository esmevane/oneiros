//! Storage view — presentation authority for the storage domain.
//!
//! Maps storage responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering layer
//! decides how to display it.

use crate::*;

pub struct StorageView;

impl StorageView {
    /// Table of storage entries with standard columns.
    pub fn table(items: &Listed<Response<StorageEntry>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("key", "Key"),
            Column::key("description", "Description").max(40),
            Column::key("hash", "Hash"),
        ]);

        for wrapped in &items.items {
            table.push_row(vec![
                wrapped.data.key.to_string(),
                wrapped.data.description.to_string(),
                wrapped.data.hash.to_string(),
            ]);
        }

        table
    }

    /// Detail view for a single storage entry.
    pub fn detail(entry: &StorageEntry) -> Detail {
        Detail::new(entry.key.to_string())
            .field("description:", entry.description.to_string())
            .field("hash:", entry.hash.to_string())
    }

    /// Confirmation for a storage mutation (e.g. "stored", "removed").
    pub fn confirmed(verb: &str, key: &StorageKey) -> Confirmation {
        Confirmation::new("Storage", key.to_string(), verb)
    }
}
