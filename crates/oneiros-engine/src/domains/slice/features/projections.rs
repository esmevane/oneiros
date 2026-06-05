use crate::*;

pub(crate) struct SliceProjections;

impl SliceProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "slice",
    migrate: |conn| SliceStore::new(conn).migrate(),
    apply: |conn, event| SliceStore::new(conn).handle(event),
    reset: |conn| SliceStore::new(conn).reset(),
}];
