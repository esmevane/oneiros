use crate::*;

pub struct PersonaProjections;

impl PersonaProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "persona",
    migrate: |conn| PersonaStore::new(conn).migrate(),
    apply: |conn, event| PersonaStore::new(conn).handle(event),
    reset: |conn| PersonaStore::new(conn).reset(),
}];
