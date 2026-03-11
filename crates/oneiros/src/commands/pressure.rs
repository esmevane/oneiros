use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum PressureError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PressureOutcomes {
    #[outcome(message("Pressure readings for '{}'.", .0.agent), prompt("{}", .0.display))]
    Readings(PressureResult),
}

#[derive(Clone, serde::Serialize)]
pub struct PressureResult {
    pub agent: AgentName,
    pub pressures: Vec<Pressure>,
    #[serde(skip)]
    pub display: String,
}

/// Show pressure readings for an agent.
#[derive(Clone, Args)]
pub struct PressureOp {
    /// The agent to read pressures for.
    name: AgentName,
}

impl PressureOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<PressureOutcomes>, PressureError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;
        let pressures = client.get_pressure(&token, &self.name).await?;
        let display = RelevantPressures::from_pressures(pressures.clone()).to_string();

        outcomes.emit(PressureOutcomes::Readings(PressureResult {
            agent: self.name.clone(),
            pressures,
            display,
        }));

        Ok(outcomes)
    }
}
