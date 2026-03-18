use crate::store::Projection;

use super::super::repo::ConnectionRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "connection",
    apply: |conn, event| ConnectionRepo::new(conn).handle(event),
    reset: |conn| ConnectionRepo::new(conn).reset(),
}];
