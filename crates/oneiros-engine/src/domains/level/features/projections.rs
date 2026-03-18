use crate::store::Projection;

use super::super::repo::LevelRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "level",
    apply: |conn, event| LevelRepo::new(conn).handle(event),
    reset: |conn| LevelRepo::new(conn).reset(),
}];
