use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SleepError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SleepOutcomes {
    #[outcome(message("'{}' is sleeping.", .0.name))]
    Sleeping(Agent),
}

/// Put an agent to sleep — record the lifecycle event and introspect.
#[derive(Clone, Args)]
pub struct SleepOp {
    /// The agent to put to sleep.
    name: AgentName,
}

impl SleepOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<SleepOutcomes>, Vec<PressureSummary>), SleepError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let response = client.sleep(&context.ticket_token()?, &self.name).await?;
        let summaries = response.pressure_summaries();
        let result: Agent = response.data()?;

        outcomes.emit(SleepOutcomes::Sleeping(result));

        Ok((outcomes, summaries))
    }
}
