use crate::*;

pub(crate) struct TrailProjections;

impl TrailProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "trail",
    migrate: |conn| TrailStore::new(conn).migrate(),
    apply: |conn, event| TrailStore::new(conn).handle(event),
    reset: |conn| TrailStore::new(conn).reset(),
}];
