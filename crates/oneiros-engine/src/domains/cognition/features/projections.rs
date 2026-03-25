use crate::*;

pub struct CognitionProjections;

impl CognitionProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "cognition",
    migrate: |conn| CognitionStore::new(conn).migrate(),
    apply: |conn, event| CognitionStore::new(conn).handle(event),
    reset: |conn| CognitionStore::new(conn).reset(),
}];
