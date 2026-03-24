use crate::*;

pub struct SearchProjections;

impl SearchProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    migrate: |conn| SearchRepo::new(conn).migrate(),
    apply: |conn, event| SearchRepo::new(conn).handle(event),
    reset: |conn| SearchRepo::new(conn).reset(),
}];
