use crate::*;

pub struct NatureProjections;

impl NatureProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "nature",
    migrate: |conn| NatureStore::new(conn).migrate(),
    apply: |conn, event| NatureStore::new(conn).handle(event),
    reset: |conn| NatureStore::new(conn).reset(),
}];
