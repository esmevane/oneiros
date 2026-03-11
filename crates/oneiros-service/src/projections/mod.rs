pub mod brain;
pub mod pressure;
pub mod search;
pub mod system;

use oneiros_db::Projection;

/// All brain-level projections: search expressions + entity materialization + pressure.
/// Search runs first so it can read entity data before brain projections mutate it.
/// Pressure runs last so it can query materialized entities.
pub const BRAIN: &[&[Projection]] = &[search::ALL, brain::ALL, pressure::ALL];

/// All system-level projections.
pub const SYSTEM: &[&[Projection]] = &[system::ALL];
