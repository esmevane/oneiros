use crate::*;

pub struct StorageProjections;

impl StorageProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "storage-set",
        migrate: |conn| StorageRepo::new(conn).migrate(),
        apply: |conn, event| StorageRepo::new(conn).handle_storage_set(event),
        reset: |conn| StorageRepo::new(conn).reset_storage(),
    },
    Projection {
        name: "storage-removed",
        migrate: |_| Ok(()),
        apply: |conn, event| StorageRepo::new(conn).handle_storage_removed(event),
        reset: |_| Ok(()),
    },
];
