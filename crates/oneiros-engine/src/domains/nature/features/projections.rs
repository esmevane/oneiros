use crate::*;

pub(crate) struct NatureProjections;

impl NatureProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "nature",
    migrate: |conn| NatureStore::new(conn).migrate(),
    apply: |conn, event| NatureStore::new(conn).handle(event),
    reset: |conn| NatureStore::new(conn).reset(),
}];
