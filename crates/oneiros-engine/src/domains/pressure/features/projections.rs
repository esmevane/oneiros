//! Pressure projections — recompute on every event.
//! This is the cross-domain projection: it reads from agents, urges,
//! and cognitions tables to derive pressure state.

use crate::store::Projection;

use super::super::repo::PressureRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "pressure",
    apply: |conn, event| PressureRepo::new(conn).handle(event),
    reset: |conn| PressureRepo::new(conn).reset(),
}];
