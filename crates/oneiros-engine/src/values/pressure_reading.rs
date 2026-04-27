use serde::{Deserialize, Serialize};

use crate::*;

/// A pressure paired with its urge's prompt — the CTA that tells an
/// agent what to do about it.
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PressureReading {
    pub pressure: Pressure,
    pub cta: Prompt,
}

impl PressureReading {
    pub fn new(pressure: Pressure, cta: Prompt) -> Self {
        Self { pressure, cta }
    }

    pub fn urgency(&self) -> f64 {
        self.pressure.urgency()
    }

    pub fn urge_name(&self) -> &UrgeName {
        &self.pressure.urge
    }

    /// Pair pressures with their urge prompts (CTAs).
    ///
    /// Each pressure is matched to its urge by name. If the urge is missing
    /// (e.g. deleted after pressure was computed), an empty prompt is used.
    pub fn from_pressures_and_urges(pressures: Vec<Pressure>, urges: &[Urge]) -> Vec<Self> {
        pressures
            .into_iter()
            .map(|p| {
                let cta = urges
                    .iter()
                    .find(|u| u.name() == &p.urge)
                    .map(|u| u.prompt().clone())
                    .unwrap_or_default();
                Self::new(p, cta)
            })
            .collect()
    }
}
