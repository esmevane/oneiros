use crate::*;

pub(crate) struct ConnectionProjections;

impl ConnectionProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "connection",
    migrate: |conn| ConnectionStore::new(conn).migrate(),
    apply: |conn, event| ConnectionStore::new(conn).handle(event),
    reset: |conn| ConnectionStore::new(conn).reset(),
}];
