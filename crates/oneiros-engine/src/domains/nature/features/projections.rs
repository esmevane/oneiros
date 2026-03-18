use crate::store::Projection;

use super::super::repo::NatureRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "nature",
    apply: |conn, event| NatureRepo::new(conn).handle(event),
    reset: |conn| NatureRepo::new(conn).reset(),
}];
