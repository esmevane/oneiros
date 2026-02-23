use oneiros_model::*;
use oneiros_outcomes::Outcome;
use std::fmt;

#[derive(Clone, serde::Serialize)]
pub struct AgentActivity {
    pub name: AgentName,
    pub cognition_count: usize,
    pub cognition_latest: Option<Timestamp>,
    pub memory_count: usize,
    pub memory_latest: Option<Timestamp>,
    pub experience_count: usize,
    pub experience_latest: Option<Timestamp>,
}

#[derive(Clone, serde::Serialize)]
pub struct AgentActivityTable {
    pub agents: Vec<AgentActivity>,
}

impl fmt::Display for AgentActivityTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.agents.is_empty() {
            return write!(f, "No agents found.");
        }

        // Calculate column widths
        let name_width = self
            .agents
            .iter()
            .map(|a| a.name.as_str().len())
            .max()
            .unwrap_or(5)
            .max(5);

        // Header
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
            let latest = [
                agent.cognition_latest,
                agent.memory_latest,
                agent.experience_latest,
            ]
            .into_iter()
            .flatten()
            .max();

            let freshness = match latest {
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

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActivityOutcomes {
    #[outcome(message("{}", .0), prompt(""))]
    Status(AgentActivityTable),
}
