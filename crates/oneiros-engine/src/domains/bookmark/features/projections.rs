use crate::*;

pub struct BookmarkProjections;

impl BookmarkProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "bookmark",
    migrate: |conn| BookmarkStore::new(conn).migrate(),
    apply: |conn, event| BookmarkStore::new(conn).handle(event),
    reset: |conn| BookmarkStore::new(conn).reset(),
}];
