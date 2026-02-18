mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use error::SleepError;
pub(crate) use outcomes::SleepOutcomes;

use crate::*;

/// Put an agent to sleep — record the lifecycle event and introspect.
#[derive(Clone, Args)]
pub(crate) struct SleepOp {
    /// The agent to put to sleep.
    name: AgentName,
}

impl SleepOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SleepOutcomes>, SleepError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let agent = client.sleep(&context.ticket_token()?, &self.name).await?;

        outcomes.emit(SleepOutcomes::Sleeping(agent));

        Ok(outcomes)
    }
}
