use crate::*;

pub struct ConnectionProjections;

impl ConnectionProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "connection",
    migrate: |conn| ConnectionRepo::new(conn).migrate(),
    apply: |conn, event| ConnectionRepo::new(conn).handle(event),
    reset: |conn| ConnectionRepo::new(conn).reset(),
}];
