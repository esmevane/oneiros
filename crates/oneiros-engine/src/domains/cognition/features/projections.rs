use crate::*;

pub struct CognitionProjections;

impl CognitionProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "cognition",
    apply: |conn, event| CognitionRepo::new(conn).handle(event),
    reset: |conn| CognitionRepo::new(conn).reset(),
}];
