use crate::*;

pub struct SensationProjections;

impl SensationProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "sensation",
    migrate: |conn| SensationRepo::new(conn).migrate(),
    apply: |conn, event| SensationRepo::new(conn).handle(event),
    reset: |conn| SensationRepo::new(conn).reset(),
}];
