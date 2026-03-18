use crate::store::Projection;

use super::super::repo::UrgeRepo;

pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "urge",
        apply: |conn, event| UrgeRepo::new(conn).handle(event),
        reset: |conn| UrgeRepo::new(conn).reset(),
    },
];
