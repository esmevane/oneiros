use crate::*;

pub(crate) struct RemoteProjections;

impl RemoteProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "remote",
    migrate: |conn| RemoteStore::new(conn).migrate(),
    apply: |conn, event| RemoteStore::new(conn).handle(event),
    reset: |conn| RemoteStore::new(conn).reset(),
}];
