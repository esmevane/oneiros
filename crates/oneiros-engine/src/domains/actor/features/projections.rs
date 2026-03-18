use crate::*;

pub struct ActorProjections;

impl ActorProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "actor",
    apply: |conn, event| ActorRepo::new(conn).handle(event),
    reset: |conn| ActorRepo::new(conn).reset(),
}];
