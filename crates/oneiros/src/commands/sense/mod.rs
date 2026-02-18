mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use oneiros_templates::SenseTemplate;
use std::io::IsTerminal;
use tokio::io::AsyncReadExt;

pub(crate) use error::SenseError;
pub(crate) use outcomes::SenseOutcomes;

use crate::*;

/// Sense an observation — interpret an external event through an agent's cognitive lens.
#[derive(Clone, Args)]
pub(crate) struct SenseOp {
    /// The agent to sense as.
    name: AgentName,
}

impl SenseOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SenseOutcomes>, SenseError> {
        let mut outcomes = Outcomes::new();

        let event_data = if std::io::stdin().is_terminal() {
            String::new()
        } else {
            let mut input = String::new();
            tokio::io::stdin().read_to_string(&mut input).await?;
            input.trim().to_string()
        };

        let client = Client::new(context.socket_path());
        let agent = client.sense(&context.ticket_token()?, &self.name).await?;
        let prompt = SenseTemplate::new(&agent, &event_data).to_string();

        outcomes.emit(SenseOutcomes::Sensing(Observation { agent, prompt }));

        Ok(outcomes)
    }
}
