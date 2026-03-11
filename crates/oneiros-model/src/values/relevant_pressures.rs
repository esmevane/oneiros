use std::fmt;

use crate::*;

/// A curated list of pressures relevant to a particular context.
///
/// For now, all pressures are relevant. The "relevant" carve-out exists
/// so that future work can filter by command, agent state, or other
/// criteria without changing the display contract.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct RelevantPressures(pub Vec<Pressure>);

impl RelevantPressures {
    pub fn from_pressures(pressures: Vec<Pressure>) -> Self {
        Self(pressures)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Compact single-line gauge for prompt attachment.
    ///
    /// Example: `[urges: introspect 72% | catharsis 12%]`
    pub fn compact(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let readings: Vec<String> = self
            .0
            .iter()
            .map(|p| format!("{} {:>2.0}%", p.urge, p.urgency() * 100.0))
            .collect();

        format!("[urges: {}]", readings.join(" | "))
    }
}

/// Detailed multi-line display for CLI output.
///
/// ```text
/// -- cognitive pressure --
/// introspect   72%   consolidate working thoughts
/// catharsis    12%   release cognitive weight
/// ```
impl fmt::Display for RelevantPressures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No pressure readings.");
        }

        writeln!(f, "-- cognitive pressure --")?;
        for pressure in &self.0 {
            writeln!(
                f,
                "{:<14} {:>3.0}%",
                pressure.urge,
                pressure.urgency() * 100.0,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pressure(urge: &str, hours: f64, cognitions: u64) -> Pressure {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: hours,
            total_cognitions: cognitions,
            working_cognitions: cognitions / 2,
            cognitions_since_introspect: cognitions,
            memories_since_introspect: 0,
            session_cognition_count: cognitions,
        };

        let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs));

        Pressure {
            id: PressureId::new(),
            agent_id: AgentId::new(),
            urge: UrgeName::new(urge),
            data: gauge,
            updated_at: Timestamp::now(),
        }
    }

    #[test]
    fn compact_empty() {
        let rp = RelevantPressures::from_pressures(vec![]);
        assert_eq!(rp.compact(), "");
    }

    #[test]
    fn compact_single() {
        let rp = RelevantPressures::from_pressures(vec![make_pressure("introspect", 4.0, 10)]);
        let compact = rp.compact();
        assert!(compact.starts_with("[urges:"));
        assert!(compact.ends_with(']'));
        assert!(compact.contains("introspect"));
        assert!(compact.contains('%'));
    }

    #[test]
    fn compact_multiple() {
        let rp = RelevantPressures::from_pressures(vec![
            make_pressure("introspect", 4.0, 10),
            make_pressure("catharsis", 0.0, 0),
        ]);
        let compact = rp.compact();
        assert!(compact.contains(" | "));
    }

    #[test]
    fn display_empty() {
        let rp = RelevantPressures::from_pressures(vec![]);
        assert_eq!(format!("{rp}"), "No pressure readings.");
    }

    #[test]
    fn display_has_header() {
        let rp = RelevantPressures::from_pressures(vec![make_pressure("introspect", 4.0, 10)]);
        let display = format!("{rp}");
        assert!(display.contains("-- cognitive pressure --"));
        assert!(display.contains("introspect"));
    }
}
