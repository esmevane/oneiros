use crate::store::Projection;

use super::super::repo::ActorRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "actor",
    apply: |conn, event| ActorRepo::new(conn).handle(event),
    reset: |conn| ActorRepo::new(conn).reset(),
}];
