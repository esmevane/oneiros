use crate::*;

pub struct FollowProjections;

impl FollowProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "follow",
    migrate: |conn| FollowStore::new(conn).migrate(),
    apply: |conn, event| FollowStore::new(conn).handle(event),
    reset: |conn| FollowStore::new(conn).reset(),
}];
