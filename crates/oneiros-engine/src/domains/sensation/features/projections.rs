use crate::store::Projection;

use super::super::repo::SensationRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "sensation",
    apply: |conn, event| SensationRepo::new(conn).handle(event),
    reset: |conn| SensationRepo::new(conn).reset(),
}];
