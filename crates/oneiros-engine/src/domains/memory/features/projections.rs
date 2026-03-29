use crate::*;

pub struct MemoryProjections;

impl MemoryProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "memory",
    migrate: |conn| MemoryRepo::new(conn).migrate(),
    apply: |conn, event| MemoryRepo::new(conn).handle(event),
    reset: |conn| MemoryRepo::new(conn).reset(),
}];
