use crate::store::Projection;

use super::super::repo::PersonaRepo;

pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "persona",
        apply: |conn, event| PersonaRepo::new(conn).handle(event),
        reset: |conn| PersonaRepo::new(conn).reset(),
    },
];
