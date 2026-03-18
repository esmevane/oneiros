use crate::store::Projection;

use super::super::repo::MemoryRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "memory",
    apply: |conn, event| MemoryRepo::new(conn).handle(event),
    reset: |conn| MemoryRepo::new(conn).reset(),
}];
