use chrono::{DateTime, Utc};
use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;
use std::fmt;

#[derive(Clone, serde::Serialize)]
pub struct AgentActivity {
    pub name: AgentName,
    pub cognition_count: usize,
    pub cognition_latest: Option<DateTime<Utc>>,
    pub memory_count: usize,
    pub memory_latest: Option<DateTime<Utc>>,
    pub experience_count: usize,
    pub experience_latest: Option<DateTime<Utc>>,
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
                Some(ts) => format_relative(ts),
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

fn format_relative(ts: DateTime<Utc>) -> String {
    let elapsed = Utc::now().signed_duration_since(ts);
    let secs = elapsed.num_seconds();

    if secs < 0 {
        "just now".to_string()
    } else if secs < 60 {
        format!("{secs}s ago")
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActivityOutcomes {
    #[outcome(message("{}", .0), prompt(""))]
    Status(AgentActivityTable),
}
