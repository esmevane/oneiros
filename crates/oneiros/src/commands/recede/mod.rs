mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use error::RecedeError;
pub(crate) use outcomes::RecedeOutcomes;

use crate::*;

/// Retire an agent — record the lifecycle event and remove.
#[derive(Clone, Args)]
pub(crate) struct RecedeOp {
    /// The agent to retire.
    name: AgentName,
}

impl RecedeOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RecedeOutcomes>, RecedeError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        client.recede(&context.ticket_token()?, &self.name).await?;

        outcomes.emit(RecedeOutcomes::Receded(self.name.clone()));

        Ok(outcomes)
    }
}
