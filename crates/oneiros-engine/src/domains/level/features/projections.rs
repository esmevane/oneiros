use crate::*;

pub struct LevelProjections;

impl LevelProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "level",
    apply: |conn, event| LevelRepo::new(conn).handle(event),
    reset: |conn| LevelRepo::new(conn).reset(),
}];
