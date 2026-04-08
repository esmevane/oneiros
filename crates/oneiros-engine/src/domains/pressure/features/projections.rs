//! Pressure projections — schema lifecycle only.
//!
//! Pressure computation moved to the reducer (PressureState).
//! The projection handles migrate/reset for the SQLite table.
//! The apply handler is a no-op — pressure data is synced to
//! SQLite by `Projections::apply` after the reducer runs.

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
    apply: |_, _| Ok(()),
    reset: |conn| PressureStore::new(conn).reset(),
}];
