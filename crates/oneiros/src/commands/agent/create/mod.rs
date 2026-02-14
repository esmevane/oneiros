mod outcomes;

use clap::Args;
use oneiros_client::{Client, CreateAgentRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::CreateAgentOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct CreateAgent {
    /// The agent name (unique identity).
    pub(crate) name: AgentName,

    /// The persona this agent adopts.
    pub(crate) persona: PersonaName,

    /// A human-readable description of the agent's purpose.
    #[arg(long, default_value = "")]
    pub(crate) description: Description,

    /// Agent-specific system prompt or instruction text.
    #[arg(long, default_value = "")]
    pub(crate) prompt: Prompt,
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

    pub(crate) async fn run(
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
        outcomes.emit(CreateAgentOutcomes::AgentCreated(info.name));

        Ok(outcomes)
    }
}
