use bon::Builder;

use crate::*;

/// Hints after waking — guide the agent into its session.
///
/// Pressure-aware: promotes Suggest to FollowUp when any pressure
/// exceeds the threshold.
#[derive(Builder)]
pub struct WakeHints {
    pub agent: AgentName,
    #[builder(default)]
    pub pressures: Vec<PressureSummary>,
}

impl WakeHints {
    pub fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        let mut hints = vec![
            Hint::follow_up(
                format!("guidebook {agent}"),
                "Review your operational context",
            ),
            Hint::inspect(
                format!("pressure {agent}"),
                "Check cognitive pressure levels",
            ),
            Hint::suggest(
                format!("cognition add {agent} observation \"...\""),
                "Record your first impression",
            ),
        ];

        if self.pressures.iter().any(|p| p.percent > 70) {
            for hint in &mut hints {
                if hint.level == HintLevel::Suggest {
                    hint.level = HintLevel::FollowUp;
                }
            }
        }

        hints
    }
}
