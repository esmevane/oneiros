use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use std::fmt;

use crate::*;

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

/// Show activity freshness for all agents across all cognitive domains.
#[derive(Clone, Args)]
pub struct ActivityStatus;

impl ActivityStatus {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ActivityOutcomes>, ActivityError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let agents = client.list_agents(&token).await?;
        let mut rows = Vec::with_capacity(agents.len());

        for agent in &agents {
            let cognitions = client
                .list_cognitions(&token, Some(&agent.name), None)
                .await
                .unwrap_or_default();
            let memories = client
                .list_memories(&token, Some(&agent.name), None)
                .await
                .unwrap_or_default();
            let experiences = client
                .list_experiences(&token, Some(&agent.name), None)
                .await
                .unwrap_or_default();

            rows.push(AgentActivity {
                name: agent.name.clone(),
                cognition_count: cognitions.len(),
                cognition_latest: most_recent(&cognitions, |c| c.created_at),
                memory_count: memories.len(),
                memory_latest: most_recent(&memories, |m| m.created_at),
                experience_count: experiences.len(),
                experience_latest: most_recent(&experiences, |e| e.created_at),
            });
        }

        outcomes.emit(ActivityOutcomes::Status(AgentActivityTable {
            agents: rows,
        }));

        Ok(outcomes)
    }
}

fn most_recent<T>(items: &[T], ts: impl Fn(&T) -> Timestamp) -> Option<Timestamp> {
    items.iter().map(ts).max()
}
