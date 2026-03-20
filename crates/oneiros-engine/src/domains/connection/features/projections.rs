use crate::*;

pub struct ConnectionProjections;

impl ConnectionProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "connection",
    apply: |conn, event| ConnectionRepo::new(conn).handle(event),
    reset: |conn| ConnectionRepo::new(conn).reset(),
}];
