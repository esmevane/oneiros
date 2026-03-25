//! Pressure projections — recompute on every event.
//! This is the cross-domain projection: it reads from agents, urges,
//! and cognitions tables to derive pressure state.

use crate::*;

pub struct PressureProjections;

impl PressureProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "pressure",
    migrate: |conn| PressureStore::new(conn).migrate(),
    apply: |conn, event| PressureStore::new(conn).handle(event),
    reset: |conn| PressureStore::new(conn).reset(),
}];
