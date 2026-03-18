//! Lifecycle has no read model — events are markers only.
//! Empty projection set for consistency with the domain pattern.

use crate::store::Projection;

pub const PROJECTIONS: &[Projection] = &[];
