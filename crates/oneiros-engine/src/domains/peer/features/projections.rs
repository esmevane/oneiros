use crate::*;

pub(crate) struct PeerProjections;

impl PeerProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "peer",
    migrate: |conn| PeerStore::new(conn).migrate(),
    apply: |conn, event| PeerStore::new(conn).handle(event),
    reset: |conn| PeerStore::new(conn).reset(),
}];
