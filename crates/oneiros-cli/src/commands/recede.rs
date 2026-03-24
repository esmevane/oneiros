use clap::Args;
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

#[derive(Clone, serde::Serialize)]
struct RecedeContext {
    name: AgentName,
    #[serde(skip)]
    prompt: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RecedeOutcomes {
    #[outcome(message("'{}' has receded.", .0.name), prompt("{}", .0.prompt))]
    Receded(RecedeContext),
}

/// Retire an agent — record the lifecycle event and remove.
#[derive(Clone, Args)]
pub struct RecedeOp {
    /// The agent to retire.
    name: AgentName,
}

impl RecedeOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<RecedeOutcomes>, Vec<PressureSummary>), RecedeError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        client.recede(&context.ticket_token()?, &self.name).await?;

        let prompt = format!(
            "Agent '{}' has receded. Their cognitions, memories, and experiences remain in the record, but they will no longer participate in active sessions.",
            self.name
        );

        outcomes.emit(RecedeOutcomes::Receded(RecedeContext {
            name: self.name.clone(),
            prompt,
        }));

        Ok((outcomes, vec![]))
    }
}
