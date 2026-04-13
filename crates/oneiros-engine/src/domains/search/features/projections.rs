use crate::*;

pub(crate) struct SearchProjections;

impl SearchProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    migrate: |conn| SearchStore::new(conn).migrate(),
    apply: |conn, event| SearchStore::new(conn).handle(event),
    reset: |conn| SearchStore::new(conn).reset(),
}];
