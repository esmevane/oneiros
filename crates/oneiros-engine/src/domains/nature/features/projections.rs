use crate::*;

pub struct NatureProjections;

impl NatureProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "nature",
    apply: |conn, event| NatureRepo::new(conn).handle(event),
    reset: |conn| NatureRepo::new(conn).reset(),
}];
