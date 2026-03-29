use crate::*;

pub struct BrainProjections;

impl BrainProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "brain",
    migrate: |conn| BrainRepo::new(conn).migrate(),
    apply: |conn, event| BrainRepo::new(conn).handle(event),
    reset: |conn| BrainRepo::new(conn).reset(),
}];
