use crate::*;

pub struct BrainProjections;

impl BrainProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "brain",
    apply: |conn, event| BrainRepo::new(conn).handle(event),
    reset: |conn| BrainRepo::new(conn).reset(),
}];
