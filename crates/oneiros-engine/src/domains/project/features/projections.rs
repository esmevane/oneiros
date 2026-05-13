use crate::*;

pub(crate) struct ProjectProjections;

impl ProjectProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "project",
    migrate: |conn| ProjectStore::new(conn).migrate(),
    apply: |conn, event| ProjectStore::new(conn).handle(event),
    reset: |conn| ProjectStore::new(conn).reset(),
}];
