use clap::Args;
use oneiros_client::{Client, CreateAgentRequest};
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum EmergeError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum EmergeOutcomes {
    #[outcome(message("'{}' has emerged.", .0))]
    Emerged(AgentName),
}

/// Bring a new agent into existence.
#[derive(Clone, Args)]
pub struct EmergeOp {
    /// The agent name (unique identity).
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

impl EmergeOp {
    fn normalize_name(&self) -> AgentName {
        let suffix = format!(".{}", self.persona.as_str());
        if self.name.as_str().ends_with(&suffix) {
            self.name.clone()
        } else {
            AgentName::new(format!("{}.{}", self.name.as_str(), self.persona.as_str()))
        }
    }

    pub async fn run(&self, context: &Context) -> Result<Outcomes<EmergeOutcomes>, EmergeError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let name = self.normalize_name();

        let agent = client
            .emerge(
                &context.ticket_token()?,
                CreateAgentRequest {
                    name,
                    persona: self.persona.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;

        outcomes.emit(EmergeOutcomes::Emerged(agent.name.clone()));

        Ok(outcomes)
    }
}
