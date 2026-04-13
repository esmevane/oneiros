use crate::*;

pub(crate) struct FollowProjections;

impl FollowProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "follow",
    migrate: |conn| FollowStore::new(conn).migrate(),
    apply: |conn, event| FollowStore::new(conn).handle(event),
    reset: |conn| FollowStore::new(conn).reset(),
}];
