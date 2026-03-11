use std::fmt;

use crate::*;

/// A curated list of pressures relevant to a particular context.
///
/// For now, all pressures are relevant. The "relevant" carve-out exists
/// so that future work can filter by command, agent state, or other
/// criteria without changing the display contract.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct RelevantPressures(pub Vec<PressureReading>);

impl RelevantPressures {
    pub fn from_readings(readings: Vec<PressureReading>) -> Self {
        Self(readings)
    }

    /// Build from bare pressures (no CTAs). Used when urge prompts are
    /// unavailable — the CTA will be empty.
    pub fn from_pressures(pressures: Vec<Pressure>) -> Self {
        Self(
            pressures
                .into_iter()
                .map(|p| PressureReading::new(p, Prompt::default()))
                .collect(),
        )
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
            .map(|r| format!("{} {:>2.0}%", r.urge_name(), r.urgency() * 100.0))
            .collect();

        format!("[urges: {}]", readings.join(" | "))
    }
}

/// Detailed multi-line display for CLI output.
///
/// ```text
/// -- cognitive pressure --
/// introspect   72%   → consolidate working thoughts
/// catharsis    12%   → release cognitive weight
/// ```
impl fmt::Display for RelevantPressures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No pressure readings.");
        }

        writeln!(f, "-- cognitive pressure --")?;
        for reading in &self.0 {
            let cta = reading.cta.as_str();
            if cta.is_empty() {
                writeln!(
                    f,
                    "{:<14} {:>3.0}%",
                    reading.urge_name(),
                    reading.urgency() * 100.0,
                )?;
            } else {
                // Truncate CTA to first sentence for display
                let short_cta = cta.split_once(". ").map_or(cta, |(first, _)| first);
                writeln!(
                    f,
                    "{:<14} {:>3.0}%   → {}",
                    reading.urge_name(),
                    reading.urgency() * 100.0,
                    short_cta,
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_reading(urge: &str, hours: f64, cognitions: u64, cta: &str) -> PressureReading {
        let inputs = IntrospectInputs {
            hours_since_last_introspect: hours,
            total_cognitions: cognitions,
            working_cognitions: cognitions / 2,
            cognitions_since_introspect: cognitions,
            memories_since_introspect: 0,
            session_cognition_count: cognitions,
        };

        let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs));

        let pressure = Pressure {
            id: PressureId::new(),
            agent_id: AgentId::new(),
            urge: UrgeName::new(urge),
            data: gauge,
            updated_at: Timestamp::now(),
        };

        PressureReading::new(pressure, Prompt::new(cta))
    }

    #[test]
    fn compact_empty() {
        let rp = RelevantPressures::from_readings(vec![]);
        assert_eq!(rp.compact(), "");
    }

    #[test]
    fn compact_single() {
        let rp =
            RelevantPressures::from_readings(vec![make_reading("introspect", 4.0, 10, "pause")]);
        let compact = rp.compact();
        assert!(compact.starts_with("[urges:"));
        assert!(compact.ends_with(']'));
        assert!(compact.contains("introspect"));
        assert!(compact.contains('%'));
    }

    #[test]
    fn compact_multiple() {
        let rp = RelevantPressures::from_readings(vec![
            make_reading("introspect", 4.0, 10, "pause"),
            make_reading("catharsis", 0.0, 0, "release"),
        ]);
        let compact = rp.compact();
        assert!(compact.contains(" | "));
    }

    #[test]
    fn display_empty() {
        let rp = RelevantPressures::from_readings(vec![]);
        assert_eq!(format!("{rp}"), "No pressure readings.");
    }

    #[test]
    fn display_has_header_and_cta() {
        let rp = RelevantPressures::from_readings(vec![make_reading(
            "introspect",
            4.0,
            10,
            "consolidate working thoughts",
        )]);
        let display = format!("{rp}");
        assert!(display.contains("-- cognitive pressure --"));
        assert!(display.contains("introspect"));
        assert!(display.contains("→ consolidate working thoughts"));
    }

    #[test]
    fn display_truncates_long_cta_to_first_sentence() {
        let rp = RelevantPressures::from_readings(vec![make_reading(
            "introspect",
            4.0,
            10,
            "First sentence. Second sentence that is longer.",
        )]);
        let display = format!("{rp}");
        assert!(display.contains("→ First sentence"));
        assert!(!display.contains("Second sentence"));
    }

    #[test]
    fn display_omits_arrow_when_no_cta() {
        let rp = RelevantPressures::from_readings(vec![make_reading("introspect", 4.0, 10, "")]);
        let display = format!("{rp}");
        assert!(!display.contains("→"));
    }

    #[test]
    fn from_pressures_creates_empty_ctas() {
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
        assert!(rp.0[0].cta.as_str().is_empty());
    }
}
