pub mod brain;
pub mod search;
pub mod system;
pub mod trust;

use oneiros_db::Projection;

/// All brain-level projections: search expressions + entity materialization.
/// Search runs first so it can read entity data before brain projections mutate it.
pub const BRAIN: &[&[Projection]] = &[search::ALL, brain::ALL];

/// All system-level projections: entity materialization + trust lifecycle.
pub const SYSTEM: &[&[Projection]] = &[system::ALL, trust::ALL];
