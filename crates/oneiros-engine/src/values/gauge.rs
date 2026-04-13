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
pub(crate) enum Gauge {
    Introspect(IntrospectGauge),
    Catharsis(CatharsisGauge),
    Recollect(RecollectGauge),
    Retrospect(RetrospectGauge),
}

impl Gauge {
    /// Compute the urgency score from this gauge's factors.
    pub(crate) fn urgency(&self) -> f64 {
        match self {
            Gauge::Introspect(g) => g.urgency(),
            Gauge::Catharsis(g) => g.urgency(),
            Gauge::Recollect(g) => g.urgency(),
            Gauge::Retrospect(g) => g.urgency(),
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
pub(crate) struct IntrospectGauge {
    pub(crate) inputs: IntrospectInputs,
    pub(crate) calculation: IntrospectCalculation,
    #[serde(default)]
    pub(crate) config: IntrospectConfig,
}

impl IntrospectGauge {
    pub(crate) fn from_inputs(inputs: IntrospectInputs) -> Self {
        Self::from_inputs_with_config(inputs, IntrospectConfig::default())
    }

    pub(crate) fn from_inputs_with_config(inputs: IntrospectInputs, config: IntrospectConfig) -> Self {
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

    pub(crate) fn urgency(&self) -> f64 {
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
pub(crate) struct IntrospectConfig {
    pub(crate) time_midpoint: f64,
    pub(crate) session_midpoint: f64,
    pub(crate) time_weight: f64,
    pub(crate) working_weight: f64,
    pub(crate) promotion_weight: f64,
    pub(crate) session_weight: f64,
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
pub(crate) struct IntrospectInputs {
    pub(crate) hours_since_last_introspect: f64,
    pub(crate) total_cognitions: u64,
    pub(crate) working_cognitions: u64,
    pub(crate) cognitions_since_introspect: u64,
    pub(crate) memories_since_introspect: u64,
    pub(crate) session_cognition_count: u64,
}

/// Computed factors derived from introspect inputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct IntrospectCalculation {
    pub(crate) time_factor: f64,
    pub(crate) working_factor: f64,
    pub(crate) promotion_factor: f64,
    pub(crate) session_factor: f64,
}

// ── Catharsis Gauge ──────────────────────────────────────────────

/// Catharsis gauge: measures tension accumulation pressure.
///
/// Four factors weighted to produce a 0–1 urgency score:
/// - tensions_factor (0.30): unresolved tensions experiences (friction accumulating)
/// - stale_working_factor (0.30): working cognitions as fraction of total (clutter)
/// - time_since_reflect_factor (0.25): hours since last reflect event
/// - orphaned_cognition_factor (0.15): cognitions not referenced by any experience
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct CatharsisGauge {
    pub(crate) inputs: CatharsisInputs,
    pub(crate) calculation: CatharsisCalculation,
    #[serde(default)]
    pub(crate) config: CatharsisConfig,
}

impl CatharsisGauge {
    pub(crate) fn from_inputs(inputs: CatharsisInputs) -> Self {
        Self::from_inputs_with_config(inputs, CatharsisConfig::default())
    }

    pub(crate) fn from_inputs_with_config(inputs: CatharsisInputs, config: CatharsisConfig) -> Self {
        let tensions_factor = saturate(
            inputs.tensions_experience_count as f64,
            config.tensions_midpoint,
        );

        let stale_working_factor = if inputs.total_cognitions > 0 {
            inputs.working_cognitions as f64 / inputs.total_cognitions as f64
        } else {
            0.0
        };

        let time_since_reflect_factor = saturate(
            inputs.hours_since_last_reflect,
            config.reflect_time_midpoint,
        );

        let orphaned_cognition_factor = if inputs.total_cognitions > 0 {
            inputs.orphaned_cognitions as f64 / inputs.total_cognitions as f64
        } else {
            0.0
        };

        let calculation = CatharsisCalculation {
            tensions_factor,
            stale_working_factor,
            time_since_reflect_factor,
            orphaned_cognition_factor,
        };

        Self {
            inputs,
            calculation,
            config,
        }
    }

    pub(crate) fn urgency(&self) -> f64 {
        self.calculation.tensions_factor * self.config.tensions_weight
            + self.calculation.stale_working_factor * self.config.stale_working_weight
            + self.calculation.time_since_reflect_factor * self.config.reflect_time_weight
            + self.calculation.orphaned_cognition_factor * self.config.orphaned_weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct CatharsisConfig {
    pub(crate) tensions_midpoint: f64,
    pub(crate) reflect_time_midpoint: f64,
    pub(crate) tensions_weight: f64,
    pub(crate) stale_working_weight: f64,
    pub(crate) reflect_time_weight: f64,
    pub(crate) orphaned_weight: f64,
}

impl Default for CatharsisConfig {
    fn default() -> Self {
        Self {
            tensions_midpoint: 3.0,
            reflect_time_midpoint: 8.0,
            tensions_weight: 0.30,
            stale_working_weight: 0.30,
            reflect_time_weight: 0.25,
            orphaned_weight: 0.15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct CatharsisInputs {
    pub(crate) tensions_experience_count: u64,
    pub(crate) total_cognitions: u64,
    pub(crate) working_cognitions: u64,
    pub(crate) hours_since_last_reflect: f64,
    pub(crate) orphaned_cognitions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct CatharsisCalculation {
    pub(crate) tensions_factor: f64,
    pub(crate) stale_working_factor: f64,
    pub(crate) time_since_reflect_factor: f64,
    pub(crate) orphaned_cognition_factor: f64,
}

// ── Recollect Gauge ─────────────────────────────────────────────

/// Recollect gauge: measures knowledge fragmentation pressure.
///
/// Four factors weighted to produce a 0–1 urgency score:
/// - session_memory_factor (0.25): session-level memories (consolidation candidates)
/// - unconnected_experience_factor (0.25): experiences not linked by connections
/// - time_since_memory_factor (0.30): hours since last memory was added
/// - working_memory_factor (0.20): working-level memories (scratchpad overflow)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RecollectGauge {
    pub(crate) inputs: RecollectInputs,
    pub(crate) calculation: RecollectCalculation,
    #[serde(default)]
    pub(crate) config: RecollectConfig,
}

impl RecollectGauge {
    pub(crate) fn from_inputs(inputs: RecollectInputs) -> Self {
        Self::from_inputs_with_config(inputs, RecollectConfig::default())
    }

    pub(crate) fn from_inputs_with_config(inputs: RecollectInputs, config: RecollectConfig) -> Self {
        let session_memory_factor = saturate(
            inputs.session_memory_count as f64,
            config.session_memory_midpoint,
        );

        let unconnected_experience_factor = if inputs.total_experiences > 0 {
            inputs.unconnected_experiences as f64 / inputs.total_experiences as f64
        } else {
            0.0
        };

        let time_since_memory_factor =
            saturate(inputs.hours_since_last_memory, config.memory_time_midpoint);

        let working_memory_factor = saturate(
            inputs.working_memory_count as f64,
            config.working_memory_midpoint,
        );

        let calculation = RecollectCalculation {
            session_memory_factor,
            unconnected_experience_factor,
            time_since_memory_factor,
            working_memory_factor,
        };

        Self {
            inputs,
            calculation,
            config,
        }
    }

    pub(crate) fn urgency(&self) -> f64 {
        self.calculation.session_memory_factor * self.config.session_memory_weight
            + self.calculation.unconnected_experience_factor
                * self.config.unconnected_experience_weight
            + self.calculation.time_since_memory_factor * self.config.memory_time_weight
            + self.calculation.working_memory_factor * self.config.working_memory_weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RecollectConfig {
    pub(crate) session_memory_midpoint: f64,
    pub(crate) memory_time_midpoint: f64,
    pub(crate) working_memory_midpoint: f64,
    pub(crate) session_memory_weight: f64,
    pub(crate) unconnected_experience_weight: f64,
    pub(crate) memory_time_weight: f64,
    pub(crate) working_memory_weight: f64,
}

impl Default for RecollectConfig {
    fn default() -> Self {
        Self {
            session_memory_midpoint: 10.0,
            memory_time_midpoint: 6.0,
            working_memory_midpoint: 8.0,
            session_memory_weight: 0.25,
            unconnected_experience_weight: 0.25,
            memory_time_weight: 0.30,
            working_memory_weight: 0.20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RecollectInputs {
    pub(crate) session_memory_count: u64,
    pub(crate) total_experiences: u64,
    pub(crate) unconnected_experiences: u64,
    pub(crate) hours_since_last_memory: f64,
    pub(crate) working_memory_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RecollectCalculation {
    pub(crate) session_memory_factor: f64,
    pub(crate) unconnected_experience_factor: f64,
    pub(crate) time_since_memory_factor: f64,
    pub(crate) working_memory_factor: f64,
}

// ── Retrospect Gauge ────────────────────────────────────────────

/// Retrospect gauge: measures arc-level reflection pressure.
///
/// Four factors weighted to produce a 0–1 urgency score:
/// - time_since_archival_factor (0.30): hours since last archival-level memory
/// - project_staleness_factor (0.25): hours since last project-level memory
/// - sessions_since_factor (0.25): wake events since last archival memory
/// - experience_accumulation_factor (0.20): total experiences (arc material)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RetrospectGauge {
    pub(crate) inputs: RetrospectInputs,
    pub(crate) calculation: RetrospectCalculation,
    #[serde(default)]
    pub(crate) config: RetrospectConfig,
}

impl RetrospectGauge {
    pub(crate) fn from_inputs(inputs: RetrospectInputs) -> Self {
        Self::from_inputs_with_config(inputs, RetrospectConfig::default())
    }

    pub(crate) fn from_inputs_with_config(inputs: RetrospectInputs, config: RetrospectConfig) -> Self {
        let time_since_archival_factor = saturate(
            inputs.hours_since_last_archival,
            config.archival_time_midpoint,
        );

        let project_staleness_factor = saturate(
            inputs.hours_since_last_project_memory,
            config.project_time_midpoint,
        );

        let sessions_since_factor = saturate(
            inputs.sessions_since_retrospect as f64,
            config.sessions_midpoint,
        );

        let experience_accumulation_factor = saturate(
            inputs.total_experience_count as f64,
            config.experience_midpoint,
        );

        let calculation = RetrospectCalculation {
            time_since_archival_factor,
            project_staleness_factor,
            sessions_since_factor,
            experience_accumulation_factor,
        };

        Self {
            inputs,
            calculation,
            config,
        }
    }

    pub(crate) fn urgency(&self) -> f64 {
        self.calculation.time_since_archival_factor * self.config.archival_time_weight
            + self.calculation.project_staleness_factor * self.config.project_staleness_weight
            + self.calculation.sessions_since_factor * self.config.sessions_weight
            + self.calculation.experience_accumulation_factor * self.config.experience_weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RetrospectConfig {
    pub(crate) archival_time_midpoint: f64,
    pub(crate) project_time_midpoint: f64,
    pub(crate) sessions_midpoint: f64,
    pub(crate) experience_midpoint: f64,
    pub(crate) archival_time_weight: f64,
    pub(crate) project_staleness_weight: f64,
    pub(crate) sessions_weight: f64,
    pub(crate) experience_weight: f64,
}

impl Default for RetrospectConfig {
    fn default() -> Self {
        Self {
            archival_time_midpoint: 48.0,
            project_time_midpoint: 24.0,
            sessions_midpoint: 5.0,
            experience_midpoint: 20.0,
            archival_time_weight: 0.30,
            project_staleness_weight: 0.25,
            sessions_weight: 0.25,
            experience_weight: 0.20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RetrospectInputs {
    pub(crate) hours_since_last_archival: f64,
    pub(crate) hours_since_last_project_memory: f64,
    pub(crate) sessions_since_retrospect: u64,
    pub(crate) total_experience_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub(crate) struct RetrospectCalculation {
    pub(crate) time_since_archival_factor: f64,
    pub(crate) project_staleness_factor: f64,
    pub(crate) sessions_since_factor: f64,
    pub(crate) experience_accumulation_factor: f64,
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

    // ── Catharsis tests ─────────────────────────────────────────

    #[test]
    fn catharsis_gauge_from_inputs() {
        let inputs = CatharsisInputs {
            tensions_experience_count: 3,
            total_cognitions: 20,
            working_cognitions: 10,
            hours_since_last_reflect: 8.0,
            orphaned_cognitions: 12,
        };

        let gauge = CatharsisGauge::from_inputs(inputs);

        // tensions: saturate(3, 3) = 0.5
        assert!((gauge.calculation.tensions_factor - 0.5).abs() < f64::EPSILON);
        // stale_working: 10/20 = 0.5
        assert!((gauge.calculation.stale_working_factor - 0.5).abs() < f64::EPSILON);
        // reflect_time: saturate(8, 8) = 0.5
        assert!((gauge.calculation.time_since_reflect_factor - 0.5).abs() < f64::EPSILON);
        // orphaned: 12/20 = 0.6
        assert!((gauge.calculation.orphaned_cognition_factor - 0.6).abs() < f64::EPSILON);

        // urgency: 0.5*0.30 + 0.5*0.30 + 0.5*0.25 + 0.6*0.15 = 0.515
        assert!((gauge.urgency() - 0.515).abs() < 0.01);
    }

    #[test]
    fn catharsis_gauge_no_tensions() {
        let inputs = CatharsisInputs {
            tensions_experience_count: 0,
            total_cognitions: 0,
            working_cognitions: 0,
            hours_since_last_reflect: 0.0,
            orphaned_cognitions: 0,
        };

        let gauge = CatharsisGauge::from_inputs(inputs);
        assert!((gauge.urgency() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn catharsis_gauge_serde_roundtrip() {
        let inputs = CatharsisInputs {
            tensions_experience_count: 2,
            total_cognitions: 10,
            working_cognitions: 5,
            hours_since_last_reflect: 4.0,
            orphaned_cognitions: 6,
        };
        let gauge = Gauge::Catharsis(CatharsisGauge::from_inputs(inputs));
        let json = serde_json::to_string(&gauge).unwrap();
        let roundtripped: Gauge = serde_json::from_str(&json).unwrap();
        assert_eq!(gauge, roundtripped);

        let json_value: serde_json::Value = serde_json::to_value(&gauge).unwrap();
        assert_eq!(json_value["type"], "catharsis");
    }

    // ── Recollect tests ─────────────────────────────────────────

    #[test]
    fn recollect_gauge_from_inputs() {
        let inputs = RecollectInputs {
            session_memory_count: 10,
            total_experiences: 20,
            unconnected_experiences: 10,
            hours_since_last_memory: 6.0,
            working_memory_count: 8,
        };

        let gauge = RecollectGauge::from_inputs(inputs);

        // session_memory: saturate(10, 10) = 0.5
        assert!((gauge.calculation.session_memory_factor - 0.5).abs() < f64::EPSILON);
        // unconnected: 10/20 = 0.5
        assert!((gauge.calculation.unconnected_experience_factor - 0.5).abs() < f64::EPSILON);
        // time: saturate(6, 6) = 0.5
        assert!((gauge.calculation.time_since_memory_factor - 0.5).abs() < f64::EPSILON);
        // working_memory: saturate(8, 8) = 0.5
        assert!((gauge.calculation.working_memory_factor - 0.5).abs() < f64::EPSILON);

        // urgency: 0.5*0.25 + 0.5*0.25 + 0.5*0.30 + 0.5*0.20 = 0.5
        assert!((gauge.urgency() - 0.5).abs() < 0.01);
    }

    #[test]
    fn recollect_gauge_no_experiences() {
        let inputs = RecollectInputs {
            session_memory_count: 0,
            total_experiences: 0,
            unconnected_experiences: 0,
            hours_since_last_memory: 0.0,
            working_memory_count: 0,
        };

        let gauge = RecollectGauge::from_inputs(inputs);
        assert!((gauge.urgency() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn recollect_gauge_serde_roundtrip() {
        let inputs = RecollectInputs {
            session_memory_count: 5,
            total_experiences: 10,
            unconnected_experiences: 3,
            hours_since_last_memory: 2.0,
            working_memory_count: 4,
        };
        let gauge = Gauge::Recollect(RecollectGauge::from_inputs(inputs));
        let json = serde_json::to_string(&gauge).unwrap();
        let roundtripped: Gauge = serde_json::from_str(&json).unwrap();
        assert_eq!(gauge, roundtripped);

        let json_value: serde_json::Value = serde_json::to_value(&gauge).unwrap();
        assert_eq!(json_value["type"], "recollect");
    }

    // ── Retrospect tests ────────────────────────────────────────

    #[test]
    fn retrospect_gauge_from_inputs() {
        let inputs = RetrospectInputs {
            hours_since_last_archival: 48.0,
            hours_since_last_project_memory: 24.0,
            sessions_since_retrospect: 5,
            total_experience_count: 20,
        };

        let gauge = RetrospectGauge::from_inputs(inputs);

        // all at midpoint → 0.5
        assert!((gauge.calculation.time_since_archival_factor - 0.5).abs() < f64::EPSILON);
        assert!((gauge.calculation.project_staleness_factor - 0.5).abs() < f64::EPSILON);
        assert!((gauge.calculation.sessions_since_factor - 0.5).abs() < f64::EPSILON);
        assert!((gauge.calculation.experience_accumulation_factor - 0.5).abs() < f64::EPSILON);

        // urgency: 0.5 * (0.30+0.25+0.25+0.20) = 0.5
        assert!((gauge.urgency() - 0.5).abs() < 0.01);
    }

    #[test]
    fn retrospect_gauge_fresh_start() {
        let inputs = RetrospectInputs {
            hours_since_last_archival: 0.0,
            hours_since_last_project_memory: 0.0,
            sessions_since_retrospect: 0,
            total_experience_count: 0,
        };

        let gauge = RetrospectGauge::from_inputs(inputs);
        assert!((gauge.urgency() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn retrospect_gauge_serde_roundtrip() {
        let inputs = RetrospectInputs {
            hours_since_last_archival: 72.0,
            hours_since_last_project_memory: 12.0,
            sessions_since_retrospect: 3,
            total_experience_count: 15,
        };
        let gauge = Gauge::Retrospect(RetrospectGauge::from_inputs(inputs));
        let json = serde_json::to_string(&gauge).unwrap();
        let roundtripped: Gauge = serde_json::from_str(&json).unwrap();
        assert_eq!(gauge, roundtripped);

        let json_value: serde_json::Value = serde_json::to_value(&gauge).unwrap();
        assert_eq!(json_value["type"], "retrospect");
    }

    // ── Cross-gauge delegation ──────────────────────────────────

    #[test]
    fn gauge_urgency_delegates_all_variants() {
        let catharsis = CatharsisGauge::from_inputs(CatharsisInputs {
            tensions_experience_count: 2,
            total_cognitions: 10,
            working_cognitions: 5,
            hours_since_last_reflect: 4.0,
            orphaned_cognitions: 3,
        });
        let gauge = Gauge::Catharsis(catharsis.clone());
        assert!((gauge.urgency() - catharsis.urgency()).abs() < f64::EPSILON);

        let recollect = RecollectGauge::from_inputs(RecollectInputs {
            session_memory_count: 5,
            total_experiences: 10,
            unconnected_experiences: 3,
            hours_since_last_memory: 2.0,
            working_memory_count: 4,
        });
        let gauge = Gauge::Recollect(recollect.clone());
        assert!((gauge.urgency() - recollect.urgency()).abs() < f64::EPSILON);

        let retrospect = RetrospectGauge::from_inputs(RetrospectInputs {
            hours_since_last_archival: 24.0,
            hours_since_last_project_memory: 12.0,
            sessions_since_retrospect: 3,
            total_experience_count: 10,
        });
        let gauge = Gauge::Retrospect(retrospect.clone());
        assert!((gauge.urgency() - retrospect.urgency()).abs() < f64::EPSILON);
    }
}
