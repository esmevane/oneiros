mod outcomes;

use clap::Args;
use oneiros_client::{Client, UpdateAgentRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::UpdateAgentOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct UpdateAgent {
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
    pub(crate) async fn run(
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
