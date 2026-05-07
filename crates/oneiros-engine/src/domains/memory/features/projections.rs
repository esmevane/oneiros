use crate::*;

pub(crate) struct MemoryProjections;

impl MemoryProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "memory",
    migrate: |conn| MemoryStore::new(conn).migrate(),
    apply: |conn, event| MemoryStore::new(conn).handle(event),
    reset: |conn| MemoryStore::new(conn).reset(),
}];
