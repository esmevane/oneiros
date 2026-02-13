mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::DreamTemplate;

pub(crate) use error::DreamError;
pub(crate) use outcomes::DreamOutcomes;

use crate::*;

/// Compose an agent's full context into a dream prompt.
#[derive(Clone, Args)]
pub(crate) struct DreamOp {
    /// The agent to dream as.
    name: AgentName,
}

impl DreamOp {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<DreamOutcomes>, DreamError> {
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
