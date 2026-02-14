mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::GuidebookTemplate;

pub(crate) use error::GuidebookError;
pub(crate) use outcomes::GuidebookOutcomes;

use crate::*;

/// Show the cognitive guidebook for an agent.
#[derive(Clone, Args)]
pub(crate) struct GuidebookOp {
    /// The agent to generate a guidebook for.
    name: AgentName,
}

impl GuidebookOp {
    pub(crate) async fn run(
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
