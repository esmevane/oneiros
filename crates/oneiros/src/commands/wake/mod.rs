mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::DreamTemplate;

pub(crate) use error::WakeError;
pub(crate) use outcomes::WakeOutcomes;

use crate::*;

/// Wake an agent — record the lifecycle event and dream.
#[derive(Clone, Args)]
pub(crate) struct WakeOp {
    /// The agent to wake.
    name: AgentName,
}

impl WakeOp {
    pub(crate) async fn run(&self, context: &Context) -> Result<Outcomes<WakeOutcomes>, WakeError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let dream_context = client.wake(&context.ticket_token()?, &self.name).await?;
        let prompt = DreamTemplate::new(&dream_context).to_string();

        outcomes.emit(WakeOutcomes::Waking(Dream {
            context: dream_context,
            prompt,
        }));

        Ok(outcomes)
    }
}
