use clap::Args;
use oneiros_client::{Client, UpdateAgentRequest};
use oneiros_model::AgentName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UpdateAgentOutcomes {
    #[outcome(message("Agent '{0}' updated."))]
    AgentUpdated(AgentName),
}

#[derive(Clone, Args)]
pub struct UpdateAgent {
    /// The agent name to update.
    name: AgentName,

    /// The persona this agent adopts.
    persona: PersonaName,

    /// A human-readable description of the agent's purpose.
    #[arg(long, default_value = "")]
    description: Description,

    /// Agent-specific system prompt or instruction text.
    #[arg(long, default_value = "")]
    prompt: Prompt,
}

impl UpdateAgent {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<UpdateAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .update_agent(
                &context.ticket_token()?,
                &self.name,
                UpdateAgentRequest {
                    persona: self.persona.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(UpdateAgentOutcomes::AgentUpdated(info.name.clone()));

        Ok(outcomes)
    }
}
