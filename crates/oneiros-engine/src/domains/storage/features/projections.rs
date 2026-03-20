use crate::*;

pub struct StorageProjections;

impl StorageProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "storage",
    apply: |conn, event| StorageRepo::new(conn).handle(event),
    reset: |conn| StorageRepo::new(conn).reset(),
}];
