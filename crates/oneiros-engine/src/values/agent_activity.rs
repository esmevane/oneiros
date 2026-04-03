//! Agent activity — a cross-agent overview of cognitive freshness.
//!
//! Shows each agent's cognition/memory/experience counts and most
//! recent activity timestamp. This is the "who's here, what have
//! they been doing" view.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// Activity summary for a single agent.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AgentActivity {
    pub name: AgentName,
    pub cognition_count: usize,
    pub cognition_latest: Option<Timestamp>,
    pub memory_count: usize,
    pub memory_latest: Option<Timestamp>,
    pub experience_count: usize,
    pub experience_latest: Option<Timestamp>,
}

impl AgentActivity {
    /// The most recent activity across all domains.
    pub fn latest(&self) -> Option<Timestamp> {
        [
            self.cognition_latest,
            self.memory_latest,
            self.experience_latest,
        ]
        .into_iter()
        .flatten()
        .max()
    }
}

/// Cross-agent activity overview — the pulse of the brain.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AgentActivityTable {
    pub agents: Vec<AgentActivity>,
}

impl core::fmt::Display for AgentActivityTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.agents.is_empty() {
            return write!(f, "No agents found.");
        }

        let name_width = self
            .agents
            .iter()
            .map(|a| a.name.as_str().len())
            .max()
            .unwrap_or(5)
            .max(5);

        writeln!(
            f,
            "{:<name_width$}  {:>5}  {:>5}  {:>5}  Last Activity",
            "Agent", "Cog", "Mem", "Exp"
        )?;
        writeln!(
            f,
            "{:<name_width$}  {:>5}  {:>5}  {:>5}  -------------",
            "-".repeat(name_width),
            "-----",
            "-----",
            "-----"
        )?;

        for agent in &self.agents {
            let freshness = match agent.latest() {
                Some(timestamp) => timestamp.elapsed(),
                None => "never".to_string(),
            };

            writeln!(
                f,
                "{:<name_width$}  {:>5}  {:>5}  {:>5}  {}",
                agent.name.as_str(),
                agent.cognition_count,
                agent.memory_count,
                agent.experience_count,
                freshness,
            )?;
        }

        Ok(())
    }
}
