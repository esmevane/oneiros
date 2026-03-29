use crate::*;

pub struct UrgeProjections;

impl UrgeProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "urge",
    migrate: |conn| UrgeRepo::new(conn).migrate(),
    apply: |conn, event| UrgeRepo::new(conn).handle(event),
    reset: |conn| UrgeRepo::new(conn).reset(),
}];
