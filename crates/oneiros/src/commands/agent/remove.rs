use clap::Args;
use oneiros_model::AgentName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveAgentOutcomes {
    #[outcome(message("Agent '{0}' removed."))]
    AgentRemoved(AgentName),
}

#[derive(Clone, Args)]
pub struct RemoveAgent {
    /// The agent name to remove.
    name: AgentName,
}

impl RemoveAgent {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        client
            .remove_agent(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveAgentOutcomes::AgentRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
