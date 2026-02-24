use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SleepError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SleepOutcomes {
    #[outcome(message("'{}' is sleeping.", .0.name))]
    Sleeping(AgentRecord),
}

/// Put an agent to sleep — record the lifecycle event and introspect.
#[derive(Clone, Args)]
pub struct SleepOp {
    /// The agent to put to sleep.
    name: AgentName,
}

impl SleepOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<SleepOutcomes>, SleepError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let agent = client.sleep(&context.ticket_token()?, &self.name).await?;

        outcomes.emit(SleepOutcomes::Sleeping(agent));

        Ok(outcomes)
    }
}
