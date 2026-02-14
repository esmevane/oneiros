mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowAgentOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowAgent {
    /// The agent name to display.
    name: AgentName,
}

impl ShowAgent {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_agent(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowAgentOutcomes::AgentDetails(info));

        Ok(outcomes)
    }
}
