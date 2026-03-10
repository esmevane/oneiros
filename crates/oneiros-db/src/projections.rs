use oneiros_model::KnownEvent;

use crate::*;

pub struct Projection {
    pub name: &'static str,
    pub apply: fn(&Database, &KnownEvent) -> Result<(), DatabaseError>,
    pub reset: fn(&Database) -> Result<(), DatabaseError>,
}

/// Project an event to all projections across multiple groups.
pub(crate) fn project(
    conn: &Database,
    projections: &[&[Projection]],
    event: &KnownEvent,
) -> Result<(), DatabaseError> {
    for group in projections {
        for projection in *group {
            (projection.apply)(conn, event)?;
        }
    }
    Ok(())
}
