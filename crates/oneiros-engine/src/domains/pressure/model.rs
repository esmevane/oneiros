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
pub(crate) struct Pressure {
    pub(crate) id: PressureId,
    pub(crate) agent_id: AgentId,
    pub(crate) urge: UrgeName,
    #[loro(json)]
    pub(crate) data: Gauge,
    pub(crate) updated_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "pressures")]
pub(crate) struct Pressures(HashMap<String, Pressure>);

impl Pressures {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Pressure> {
        self.0.values()
    }

    pub(crate) fn get(&self, id: PressureId) -> Option<&Pressure> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, pressure: &Pressure) -> Option<Pressure> {
        self.0.insert(pressure.id.to_string(), pressure.clone())
    }

    pub(crate) fn remove(&mut self, pressure_id: PressureId) -> Option<Pressure> {
        self.0.remove(&pressure_id.to_string())
    }
}

impl Pressure {
    /// Compute urgency as a 0.0-1.0 score from the stored gauge.
    pub(crate) fn urgency(&self) -> f64 {
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
