use crate::*;

pub struct PersonaProjections;

impl PersonaProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "persona",
    apply: |conn, event| PersonaRepo::new(conn).handle(event),
    reset: |conn| PersonaRepo::new(conn).reset(),
}];
