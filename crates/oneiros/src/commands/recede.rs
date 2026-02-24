use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum RecedeError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RecedeOutcomes {
    #[outcome(message("'{}' has receded.", .0))]
    Receded(AgentName),
}

/// Retire an agent — record the lifecycle event and remove.
#[derive(Clone, Args)]
pub struct RecedeOp {
    /// The agent to retire.
    name: AgentName,
}

impl RecedeOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<RecedeOutcomes>, RecedeError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        client.recede(&context.ticket_token()?, &self.name).await?;

        outcomes.emit(RecedeOutcomes::Receded(self.name.clone()));

        Ok(outcomes)
    }
}
