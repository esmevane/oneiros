use crate::*;

pub(crate) struct ActorProjections;

impl ActorProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "actor",
    migrate: |conn| ActorStore::new(conn).migrate(),
    apply: |conn, event| ActorStore::new(conn).handle(event),
    reset: |conn| ActorStore::new(conn).reset(),
}];
