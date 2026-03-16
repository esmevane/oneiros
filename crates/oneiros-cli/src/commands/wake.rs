use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::DreamTemplate;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum WakeError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum WakeOutcomes {
    #[outcome(message("Waking as '{}'...", .0.context.agent.name), prompt("{}", .0.prompt))]
    Waking(Dream),
}

/// Wake an agent — record the lifecycle event and dream.
#[derive(Clone, Args)]
pub(crate) struct WakeOp {
    /// The agent to wake.
    name: AgentName,
}

impl WakeOp {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<WakeOutcomes>, Vec<PressureSummary>), WakeError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let response = client.wake(&context.ticket_token()?, &self.name).await?;
        let summaries = response.pressure_summaries();
        let dream_context: DreamContext = response.data()?;
        let prompt = DreamTemplate::new(&dream_context).to_string();

        outcomes.emit(WakeOutcomes::Waking(Dream {
            context: dream_context,
            prompt,
        }));

        Ok((outcomes, summaries))
    }
}
