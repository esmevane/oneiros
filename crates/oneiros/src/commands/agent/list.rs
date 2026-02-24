use clap::Args;
use oneiros_client::Client;
use oneiros_model::AgentRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListAgentsOutcomes {
    #[outcome(message("No agents configured."))]
    NoAgents,

    #[outcome(message("Agents: {0:?}"))]
    Agents(Vec<AgentRecord>),
}

#[derive(Clone, Args)]
pub struct ListAgents;

impl ListAgents {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListAgentsOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let agents = client.list_agents(&context.ticket_token()?).await?;

        if agents.is_empty() {
            outcomes.emit(ListAgentsOutcomes::NoAgents);
        } else {
            outcomes.emit(ListAgentsOutcomes::Agents(agents));
        }

        Ok(outcomes)
    }
}
