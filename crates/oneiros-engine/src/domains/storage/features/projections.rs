use crate::store::Projection;

use super::super::repo::StorageRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "storage",
    apply: |conn, event| StorageRepo::new(conn).handle(event),
    reset: |conn| StorageRepo::new(conn).reset(),
}];
