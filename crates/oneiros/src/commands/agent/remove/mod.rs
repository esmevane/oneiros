mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveAgentOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveAgent {
    /// The agent name to remove.
    name: AgentName,
}

impl RemoveAgent {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_agent(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveAgentOutcomes::AgentRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
