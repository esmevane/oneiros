use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateAgentOutcomes {
    #[outcome(message("Agent '{0}' created."))]
    AgentCreated(AgentName),
}

#[derive(Clone, Args)]
pub struct CreateAgent {
    /// The agent name (unique identity).
    pub name: AgentName,

    /// The persona this agent adopts.
    pub persona: PersonaName,

    /// A human-readable description of the agent's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Agent-specific system prompt or instruction text.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl CreateAgent {
    /// Normalize the agent name to include the persona suffix.
    ///
    /// - If name already ends with `.{persona}`, use as-is
    /// - Otherwise, append `.{persona}` to the name
    fn normalize_name(&self) -> AgentName {
        let suffix = format!(".{}", self.persona.as_str());
        if self.name.as_str().ends_with(&suffix) {
            self.name.clone()
        } else {
            AgentName::new(format!("{}.{}", self.name.as_str(), self.persona.as_str()))
        }
    }

    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateAgentOutcomes>, AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let name = self.normalize_name();

        let info = client
            .create_agent(
                &context.ticket_token()?,
                CreateAgentRequest {
                    name,
                    persona: self.persona.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(CreateAgentOutcomes::AgentCreated(info.name.clone()));

        Ok(outcomes)
    }
}
