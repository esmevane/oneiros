use crate::*;

pub(crate) struct PersonaProjections;

impl PersonaProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "persona",
    migrate: |conn| PersonaStore::new(conn).migrate(),
    apply: |conn, event| PersonaStore::new(conn).handle(event),
    reset: |conn| PersonaStore::new(conn).reset(),
}];
