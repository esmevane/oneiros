use crate::*;

pub struct BrainProjections;

impl BrainProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "brain",
    migrate: |conn| BrainStore::new(conn).migrate(),
    apply: |conn, event| BrainStore::new(conn).handle(event),
    reset: |conn| BrainStore::new(conn).reset(),
}];
