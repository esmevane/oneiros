use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

resource_id!(PressureId);

/// A pressure reading — derived state computed from cross-domain queries.
///
/// Each pressure pairs an agent with an urge and stores the full gauge
/// data (inputs, calculation, config) as a self-describing audit trail.
/// Urgency is computed at read time from the stored gauge.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pressure {
    pub id: PressureId,
    pub agent_id: AgentId,
    pub urge: UrgeName,
    pub data: Gauge,
    pub updated_at: Timestamp,
}

impl Pressure {
    /// Compute urgency as a 0.0-1.0 score from the stored gauge.
    pub fn urgency(&self) -> f64 {
        self.data.urgency()
    }
}

/// Compact pressure summary for response metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PressureSummary {
    pub urge: UrgeName,
    pub percent: u8,
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
