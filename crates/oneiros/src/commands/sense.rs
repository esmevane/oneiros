use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::SenseTemplate;
use std::io::IsTerminal;
use tokio::io::AsyncReadExt;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SenseError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Failed to read from stdin: {0}")]
    Stdin(#[from] std::io::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SenseOutcomes {
    #[outcome(message("Sensing as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Sensing(Observation),
}

/// Sense an observation — interpret an external event through an agent's cognitive lens.
#[derive(Clone, Args)]
pub struct SenseOp {
    /// The agent to sense as.
    name: AgentName,
}

impl SenseOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<SenseOutcomes>, SenseError> {
        let mut outcomes = Outcomes::new();

        let event_data = if std::io::stdin().is_terminal() {
            String::new()
        } else {
            let mut input = String::new();
            tokio::io::stdin().read_to_string(&mut input).await?;
            input.trim().to_string()
        };

        let client = context.client();
        let agent = client.sense(&context.ticket_token()?, &self.name).await?;
        let prompt = SenseTemplate::new(&agent, &event_data).to_string();

        outcomes.emit(SenseOutcomes::Sensing(Observation { agent, prompt }));

        Ok(outcomes)
    }
}
