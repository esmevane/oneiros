use crate::*;

pub(crate) struct StorageProjections;

impl StorageProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "storage",
    migrate: |conn| StorageStore::new(conn).migrate(),
    apply: |conn, event| StorageStore::new(conn).handle(event),
    reset: |conn| StorageStore::new(conn).reset_storage(),
}];
