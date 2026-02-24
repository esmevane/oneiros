use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::GuidebookTemplate;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum GuidebookError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum GuidebookOutcomes {
    #[outcome(message("Guidebook for '{}'", .0.context.agent.name), prompt("{}", .0.prompt))]
    Guidebook(Dream),
}

/// Show the cognitive guidebook for an agent.
#[derive(Clone, Args)]
pub struct GuidebookOp {
    /// The agent to generate a guidebook for.
    name: AgentName,
}

impl GuidebookOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<GuidebookOutcomes>, GuidebookError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let dream_context = client.dream(&context.ticket_token()?, &self.name).await?;
        let prompt = GuidebookTemplate::new(&dream_context).to_string();

        outcomes.emit(GuidebookOutcomes::Guidebook(Dream {
            context: dream_context,
            prompt,
        }));

        Ok(outcomes)
    }
}
