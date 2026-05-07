use crate::*;

pub(crate) struct LevelProjections;

impl LevelProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "level",
    migrate: |conn| LevelStore::new(conn).migrate(),
    apply: |conn, event| LevelStore::new(conn).handle(event),
    reset: |conn| LevelStore::new(conn).reset(),
}];
