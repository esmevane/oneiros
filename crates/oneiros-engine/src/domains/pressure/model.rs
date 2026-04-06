use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

resource_id!(PressureId);

/// A pressure reading — derived state computed from cross-domain queries.
///
/// Each pressure pairs an agent with an urge and stores the full gauge
/// data (inputs, calculation, config) as a self-describing audit trail.
/// Urgency is computed at read time from the stored gauge.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Hydrate, Reconcile)]
pub struct Pressure {
    pub id: PressureId,
    pub agent_id: AgentId,
    pub urge: UrgeName,
    #[loro(json)]
    pub data: Gauge,
    pub updated_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "pressures")]
pub struct Pressures(HashMap<String, Pressure>);

impl Pressure {
    /// Compute urgency as a 0.0-1.0 score from the stored gauge.
    pub fn urgency(&self) -> f64 {
        self.data.urgency()
    }
}

impl From<&Pressure> for PressureSummary {
    fn from(p: &Pressure) -> Self {
        let raw = (p.urgency() * 100.0).round();
        let percent = raw.clamp(0.0, 100.0) as u8;
        Self {
            urge: p.urge.clone(),
            percent,
        }
    }
}
