mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use error::StatusError;
pub(crate) use outcomes::{StatusOutcomes, StatusResult};

use crate::*;

/// Show a full cognitive status dashboard for an agent.
#[derive(Clone, Args)]
pub(crate) struct StatusOp {
    /// The agent to show status for.
    agent: AgentName,
}

impl StatusOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<StatusOutcomes>, StatusError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let cognitions = client
            .list_cognitions(&token, Some(&self.agent), None)
            .await?;
        let memories = client
            .list_memories(&token, Some(&self.agent), None)
            .await?;
        let experiences = client
            .list_experiences(&token, Some(&self.agent), None)
            .await?;

        let dashboard =
            crate::gauge::full_status(&self.agent, &cognitions, &memories, &experiences);

        outcomes.emit(StatusOutcomes::Status(StatusResult { dashboard }));

        Ok(outcomes)
    }
}
