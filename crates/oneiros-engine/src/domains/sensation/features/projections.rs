use crate::*;

pub(crate) struct SensationProjections;

impl SensationProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "sensation",
    migrate: |conn| SensationStore::new(conn).migrate(),
    apply: |conn, event| SensationStore::new(conn).handle(event),
    reset: |conn| SensationStore::new(conn).reset(),
}];
