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
        migrate: |conn| StorageRepo::new(conn).migrate(),
        apply: |conn, event| StorageRepo::new(conn).handle_blob_stored(event),
        reset: |conn| StorageRepo::new(conn).reset_blobs(),
    },
    Projection {
        name: "storage-set",
        migrate: |_| Ok(()), // Schema owned by blob-stored projection
        apply: |conn, event| StorageRepo::new(conn).handle_storage_set(event),
        reset: |conn| StorageRepo::new(conn).reset_storage(),
    },
    Projection {
        name: "storage-removed",
        migrate: |_| Ok(()), // Schema owned by blob-stored projection
        apply: |conn, event| StorageRepo::new(conn).handle_storage_removed(event),
        reset: |_| Ok(()), // Naturally idempotent
    },
];
