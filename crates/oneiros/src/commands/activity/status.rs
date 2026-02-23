use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

use super::error::ActivityError;
use super::outcomes::{ActivityOutcomes, AgentActivity, AgentActivityTable};
use crate::*;

/// Show activity freshness for all agents across all cognitive domains.
#[derive(Clone, Args)]
pub(crate) struct ActivityStatus;

impl ActivityStatus {
    pub(crate) async fn run(
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
