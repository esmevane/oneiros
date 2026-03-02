use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum StatusError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize)]
pub struct StatusResult {
    #[serde(skip)]
    pub dashboard: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StatusOutcomes {
    #[outcome(message("Status retrieved."), prompt("{}", .0.dashboard))]
    Status(StatusResult),
}

/// Show a full cognitive status dashboard for an agent.
#[derive(Clone, Args)]
pub struct StatusOp {
    /// The agent to show status for.
    agent: AgentName,
}

impl StatusOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<StatusOutcomes>, StatusError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
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
