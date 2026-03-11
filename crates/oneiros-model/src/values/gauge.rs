use serde::{Deserialize, Serialize};

/// Smooth saturation function: maps [0, ∞) → [0, 1).
/// At value == midpoint, output == 0.5.
fn saturate(value: f64, midpoint: f64) -> f64 {
    if midpoint <= 0.0 {
        return 1.0;
    }
    value / (value + midpoint)
}

/// Self-describing gauge data stored in a pressure record's `data` column.
/// Each variant carries its own inputs and computed factors, providing a
/// complete audit trail of how urgency was derived.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum Gauge {
    Introspect(IntrospectGauge),
}

impl Gauge {
    /// Compute the urgency score from this gauge's factors.
    pub fn urgency(&self) -> f64 {
        match self {
            Gauge::Introspect(g) => g.urgency(),
        }
    }
}

/// Introspect gauge: measures cognitive consolidation pressure.
///
/// Four factors weighted to produce a 0–1 urgency score:
/// - time_since_last_introspect (0.30): hours since last introspection
/// - working_fraction (0.30): ratio of working-texture cognitions to total
/// - memory_promotion_rate (0.20): inverted — low promotion = high pressure
/// - session_cognition_count (0.20): cognitions since last wake
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct IntrospectGauge {
    pub inputs: IntrospectInputs,
    pub calculation: IntrospectCalculation,
    #[serde(default)]
    pub config: IntrospectConfig,
}

impl IntrospectGauge {
    pub fn from_inputs(inputs: IntrospectInputs) -> Self {
        Self::from_inputs_with_config(inputs, IntrospectConfig::default())
    }

    pub fn from_inputs_with_config(inputs: IntrospectInputs, config: IntrospectConfig) -> Self {
        let time_factor = saturate(inputs.hours_since_last_introspect, config.time_midpoint);

        let working_factor = if inputs.total_cognitions > 0 {
            inputs.working_cognitions as f64 / inputs.total_cognitions as f64
        } else {
            0.0
        };

        let promotion_rate = if inputs.cognitions_since_introspect > 0 {
            inputs.memories_since_introspect as f64 / inputs.cognitions_since_introspect as f64
        } else {
            0.0
        };
        let promotion_factor = 1.0 - promotion_rate;

        let session_factor = saturate(
            inputs.session_cognition_count as f64,
            config.session_midpoint,
        );

        let calculation = IntrospectCalculation {
            time_factor,
            working_factor,
            promotion_factor,
            session_factor,
        };

        Self {
            inputs,
            calculation,
            config,
        }
    }

    pub fn urgency(&self) -> f64 {
        self.calculation.time_factor * self.config.time_weight
            + self.calculation.working_factor * self.config.working_weight
            + self.calculation.promotion_factor * self.config.promotion_weight
            + self.calculation.session_factor * self.config.session_weight
    }
}

/// Tuning parameters for the introspect gauge.
///
/// TODO: Thread from runtime config (oneiros-config) through the projection
/// stack so these can be adjusted without code changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct IntrospectConfig {
    pub time_midpoint: f64,
    pub session_midpoint: f64,
    pub time_weight: f64,
    pub working_weight: f64,
    pub promotion_weight: f64,
    pub session_weight: f64,
}

impl Default for IntrospectConfig {
    fn default() -> Self {
        Self {
            time_midpoint: 4.0,
            session_midpoint: 15.0,
            time_weight: 0.30,
            working_weight: 0.30,
            promotion_weight: 0.20,
            session_weight: 0.20,
        }
    }
}

/// Raw inputs gathered by the projection for the introspect heuristic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct IntrospectInputs {
    pub hours_since_last_introspect: f64,
    pub total_cognitions: u64,
    pub working_cognitions: u64,
    pub cognitions_since_introspect: u64,
    pub memories_since_introspect: u64,
    pub session_cognition_count: u64,
}

/// Computed factors derived from introspect inputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct IntrospectCalculation {
    pub time_factor: f64,
    pub working_factor: f64,
    pub promotion_factor: f64,
    pub session_factor: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturate_at_zero() {
        assert!((saturate(0.0, 4.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn saturate_at_midpoint() {
        assert!((saturate(4.0, 4.0) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn saturate_large_value() {
        let result = saturate(100.0, 4.0);
        assert!(result > 0.95);
        assert!(result < 1.0);
    }

    #[test]
    fn saturate_zero_midpoint() {
        assert!((saturate(5.0, 0.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn introspect_gauge_from_inputs() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 4.0,
            total_cognitions: 10,
            working_cognitions: 5,
            cognitions_since_introspect: 10,
            memories_since_introspect: 2,
            session_cognition_count: 15,
        };

        let gauge = IntrospectGauge::from_inputs(inputs);

        // time: saturate(4, 4) = 0.5
        assert!((gauge.calculation.time_factor - 0.5).abs() < f64::EPSILON);
        // working: 5/10 = 0.5
        assert!((gauge.calculation.working_factor - 0.5).abs() < f64::EPSILON);
        // promotion: 1 - (2/10) = 0.8
        assert!((gauge.calculation.promotion_factor - 0.8).abs() < f64::EPSILON);
        // session: saturate(15, 15) = 0.5
        assert!((gauge.calculation.session_factor - 0.5).abs() < f64::EPSILON);

        // urgency: 0.5*0.30 + 0.5*0.30 + 0.8*0.20 + 0.5*0.20 = 0.56
        assert!((gauge.urgency() - 0.56).abs() < 0.01);
    }

    #[test]
    fn introspect_gauge_no_cognitions() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 24.0,
            total_cognitions: 0,
            working_cognitions: 0,
            cognitions_since_introspect: 0,
            memories_since_introspect: 0,
            session_cognition_count: 0,
        };

        let gauge = IntrospectGauge::from_inputs(inputs);

        // working = 0, session = 0, promotion inverted = 1.0
        assert!((gauge.calculation.working_factor - 0.0).abs() < f64::EPSILON);
        assert!((gauge.calculation.session_factor - 0.0).abs() < f64::EPSILON);
        assert!((gauge.calculation.promotion_factor - 1.0).abs() < f64::EPSILON);
        // time: saturate(24, 4) = 24/28 ≈ 0.857
        assert!(gauge.calculation.time_factor > 0.85);

        assert!(gauge.urgency() > 0.0);
        assert!(gauge.urgency() < 1.0);
    }

    #[test]
    fn gauge_urgency_delegates() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 0.0,
            total_cognitions: 0,
            working_cognitions: 0,
            cognitions_since_introspect: 0,
            memories_since_introspect: 0,
            session_cognition_count: 0,
        };

        let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs.clone()));
        let inner = IntrospectGauge::from_inputs(inputs);

        assert!((gauge.urgency() - inner.urgency()).abs() < f64::EPSILON);
    }

    #[test]
    fn gauge_serde_roundtrip() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 2.5,
            total_cognitions: 20,
            working_cognitions: 8,
            cognitions_since_introspect: 15,
            memories_since_introspect: 3,
            session_cognition_count: 10,
        };

        let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs));
        let json = serde_json::to_string(&gauge).unwrap();
        let roundtripped: Gauge = serde_json::from_str(&json).unwrap();

        assert_eq!(gauge, roundtripped);
    }

    #[test]
    fn gauge_json_has_type_tag() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 1.0,
            total_cognitions: 5,
            working_cognitions: 2,
            cognitions_since_introspect: 5,
            memories_since_introspect: 1,
            session_cognition_count: 3,
        };

        let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs));
        let json: serde_json::Value = serde_json::to_value(&gauge).unwrap();

        assert_eq!(json["type"], "introspect");
        assert!(json["data"]["inputs"].is_object());
        assert!(json["data"]["calculation"].is_object());
    }
}
