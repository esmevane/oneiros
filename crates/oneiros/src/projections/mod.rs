mod actor;
mod tenant;

use oneiros_db::Projection;

/// System projections, ordered by dependency (tenant before actor).
pub(crate) const SYSTEM_PROJECTIONS: &[Projection] = &[tenant::PROJECTION, actor::PROJECTION];
