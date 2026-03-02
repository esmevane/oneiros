use clap::Args;
use oneiros_model::Agent;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowAgentOutcomes {
    #[outcome(message("Agent '{}' (persona: {})\n  Description: {}\n  Prompt: {}", .0.name, .0.persona, .0.description, .0.prompt))]
    AgentDetails(Agent),
}

#[derive(Clone, Args)]
pub struct ShowAgent {
    /// The agent name to display.
    name: AgentName,
}

impl ShowAgent {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info = client
            .get_agent(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowAgentOutcomes::AgentDetails(info));

        Ok(outcomes)
    }
}
