use crate::store::Projection;

use super::super::repo::BrainRepo;

pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "brain",
        apply: |conn, event| BrainRepo::new(conn).handle(event),
        reset: |conn| BrainRepo::new(conn).reset(),
    },
];
