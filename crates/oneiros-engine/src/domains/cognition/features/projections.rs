use crate::store::Projection;

use super::super::repo::CognitionRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "cognition",
    apply: |conn, event| CognitionRepo::new(conn).handle(event),
    reset: |conn| CognitionRepo::new(conn).reset(),
}];
