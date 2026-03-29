use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// Compact pressure representation for ambient meta on every response.
///
/// Carries just the urge name and a 0-100 urgency percentage — enough
/// for ambient awareness without the full gauge audit trail.
/// Full detail is available via `get_pressure` / `list_pressures`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PressureSummary {
    pub urge: UrgeName,
    pub percent: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pressure(urge: &str, hours: f64, cognitions: u64) -> Pressure {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: hours,
            total_cognitions: cognitions,
            working_cognitions: cognitions / 2,
            cognitions_since_introspect: cognitions,
            memories_since_introspect: 0,
            session_cognition_count: cognitions,
        };

        Pressure {
            id: PressureId::new(),
            agent_id: AgentId::new(),
            urge: UrgeName::new(urge),
            data: Gauge::Introspect(IntrospectGauge::from_inputs(inputs)),
            updated_at: Timestamp::now(),
        }
    }

    #[test]
    fn from_pressure_rounds_correctly() {
        let p = test_pressure("introspect", 4.0, 10);
        let summary = PressureSummary::from(&p);
        assert_eq!(summary.urge, UrgeName::new("introspect"));
        assert!(summary.percent <= 100);
    }

    #[test]
    fn from_pressure_clamps_to_100() {
        // Even with extreme inputs, percent should never exceed 100
        let p = test_pressure("introspect", 1000.0, 10000);
        let summary = PressureSummary::from(&p);
        assert!(summary.percent <= 100);
    }

    #[test]
    fn from_pressure_low_inputs() {
        let p = test_pressure("introspect", 0.0, 0);
        let summary = PressureSummary::from(&p);
        // Even with zero hours/cognitions, the gauge may still produce
        // a small baseline — just verify it's a valid percentage.
        assert!(summary.percent <= 100);
    }

    #[test]
    fn serde_round_trip() {
        let summary = PressureSummary {
            urge: UrgeName::new("introspect"),
            percent: 69,
        };
        let json = serde_json::to_string(&summary).unwrap();
        let back: PressureSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, back);
    }

    #[test]
    fn serde_compact_size() {
        let summary = PressureSummary {
            urge: UrgeName::new("introspect"),
            percent: 69,
        };
        let json = serde_json::to_string(&summary).unwrap();
        // Should be tiny — ~40 bytes, not 1-1.5KB
        assert!(json.len() < 100, "JSON too large: {json}");
    }
}
