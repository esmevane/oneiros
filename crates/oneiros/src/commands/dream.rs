use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::DreamTemplate;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum DreamError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DreamOutcomes {
    #[outcome(message("Dreaming as '{}'...", .0.context.agent.name), prompt("{}", .0.prompt))]
    Dreaming(Dream),
}

/// Compose an agent's full context into a dream prompt.
#[derive(Clone, Args)]
pub struct DreamOp {
    /// The agent to dream as.
    name: AgentName,
}

impl DreamOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<DreamOutcomes>, DreamError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let dream_context = client.dream(&context.ticket_token()?, &self.name).await?;
        let prompt = DreamTemplate::new(&dream_context).to_string();

        outcomes.emit(DreamOutcomes::Dreaming(Dream {
            context: dream_context,
            prompt,
        }));

        Ok(outcomes)
    }
}
