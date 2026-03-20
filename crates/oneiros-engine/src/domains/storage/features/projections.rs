use crate::*;

pub struct StorageProjections;

impl StorageProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "blob-stored",
        apply: |conn, event| StorageRepo::new(conn).handle_blob_stored(event),
        reset: |conn| StorageRepo::new(conn).reset_blobs(),
    },
    Projection {
        name: "storage-set",
        apply: |conn, event| StorageRepo::new(conn).handle_storage_set(event),
        reset: |conn| StorageRepo::new(conn).reset_storage(),
    },
    Projection {
        name: "storage-removed",
        apply: |conn, event| StorageRepo::new(conn).handle_storage_removed(event),
        reset: |_| Ok(()), // Naturally idempotent
    },
];
