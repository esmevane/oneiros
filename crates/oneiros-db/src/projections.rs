use serde_json::Value;

use crate::*;

pub struct Projection {
    pub name: &'static str,
    pub events: &'static [&'static str],
    pub apply: fn(&Database, &Value) -> Result<(), DatabaseError>,
    pub reset: fn(&Database) -> Result<(), DatabaseError>,
}

/// Project an event to all relevant projections.
pub(crate) fn project(
    conn: &Database,
    projections: &[Projection],
    event_type: &str,
    data: &Value,
) -> Result<(), DatabaseError> {
    for projection in projections {
        if projection.events.contains(&event_type) {
            (projection.apply)(conn, data)?;
        }
    }
    Ok(())
}
