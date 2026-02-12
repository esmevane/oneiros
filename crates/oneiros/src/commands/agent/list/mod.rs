mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListAgentsOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListAgents;

impl ListAgents {
    pub(crate) async fn run(
        &self,
        context: Context,
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
