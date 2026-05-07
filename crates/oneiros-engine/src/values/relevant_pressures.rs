use std::fmt;

use crate::*;

/// A curated list of pressures relevant to a particular context.
///
/// For now, all pressures are relevant. The "relevant" carve-out exists
/// so that future work can filter by command, agent state, or other
/// criteria without changing the display contract.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct RelevantPressures(pub(crate) Vec<PressureSummary>);

impl RelevantPressures {
    pub(crate) fn from_summaries(summaries: Vec<PressureSummary>) -> Self {
        Self(summaries)
    }

    /// Build from bare pressures. Used when urge prompts are
    /// unavailable — converts via PressureSummary.
    pub(crate) fn from_pressures(pressures: Vec<Pressure>) -> Self {
        Self(pressures.iter().map(PressureSummary::from).collect())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Compact single-line gauge for prompt attachment.
    ///
    /// Example: `[urges: introspect 72% | catharsis 12%]`
    pub(crate) fn compact(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let readings: Vec<String> = self
            .0
            .iter()
            .map(|s| format!("{} {:>2}%", s.urge, s.percent))
            .collect();

        format!("[urges: {}]", readings.join(" | "))
    }
}

/// Detailed multi-line display for CLI output.
///
/// ```text
/// -- cognitive pressure --
/// introspect   72%
/// catharsis    12%
/// ```
impl fmt::Display for RelevantPressures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No pressure readings.");
        }

        writeln!(f, "-- cognitive pressure --")?;
        for summary in &self.0 {
            writeln!(f, "{:<14} {:>3}%", summary.urge, summary.percent)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_summary(urge: &str, percent: u8) -> PressureSummary {
        PressureSummary {
            urge: UrgeName::new(urge),
            percent,
        }
    }

    #[test]
    fn compact_empty() {
        let rp = RelevantPressures::from_summaries(vec![]);
        assert_eq!(rp.compact(), "");
    }

    #[test]
    fn compact_single() {
        let rp = RelevantPressures::from_summaries(vec![make_summary("introspect", 72)]);
        let compact = rp.compact();
        assert!(compact.starts_with("[urges:"));
        assert!(compact.ends_with(']'));
        assert!(compact.contains("introspect"));
        assert!(compact.contains("72%"));
    }

    #[test]
    fn compact_multiple() {
        let rp = RelevantPressures::from_summaries(vec![
            make_summary("introspect", 72),
            make_summary("catharsis", 12),
        ]);
        let compact = rp.compact();
        assert!(compact.contains(" | "));
    }

    #[test]
    fn display_empty() {
        let rp = RelevantPressures::from_summaries(vec![]);
        assert_eq!(format!("{rp}"), "No pressure readings.");
    }

    #[test]
    fn display_has_header() {
        let rp = RelevantPressures::from_summaries(vec![make_summary("introspect", 72)]);
        let display = format!("{rp}");
        assert!(display.contains("-- cognitive pressure --"));
        assert!(display.contains("introspect"));
        assert!(display.contains("72%"));
    }

    #[test]
    fn from_pressures_converts() {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: 4.0,
            total_cognitions: 10,
            working_cognitions: 5,
            cognitions_since_introspect: 10,
            memories_since_introspect: 0,
            session_cognition_count: 10,
        };
        let pressure = Pressure {
            id: PressureId::new(),
            agent_id: AgentId::new(),
            urge: UrgeName::new("introspect"),
            data: Gauge::Introspect(IntrospectGauge::from_inputs(inputs)),
            updated_at: Timestamp::now(),
        };

        let rp = RelevantPressures::from_pressures(vec![pressure]);
        assert_eq!(rp.0.len(), 1);
        assert!(rp.0[0].percent <= 100);
    }
}
