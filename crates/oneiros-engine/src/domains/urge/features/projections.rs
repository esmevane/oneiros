use crate::*;

pub(crate) struct UrgeProjections;

impl UrgeProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "urge",
    migrate: |conn| UrgeStore::new(conn).migrate(),
    apply: |conn, event| UrgeStore::new(conn).handle(event),
    reset: |conn| UrgeStore::new(conn).reset(),
}];
