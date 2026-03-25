use clap::Args;
use oneiros_model::{Agent, PressureSummary};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListAgentsOutcomes {
    #[outcome(message("No agents configured."), prompt("No agents configured."))]
    NoAgents,

    #[outcome(message("Agents: {0:?}"), prompt("Agents: {0:?}"))]
    Agents(Vec<Agent>),
}

#[derive(Clone, Args)]
pub struct ListAgents;

impl ListAgents {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ListAgentsOutcomes>, Vec<PressureSummary>), AgentCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client.list_agents(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let agents: Vec<Agent> = response.data()?;

        if agents.is_empty() {
            outcomes.emit(ListAgentsOutcomes::NoAgents);
        } else {
            outcomes.emit(ListAgentsOutcomes::Agents(agents));
        }

        Ok((outcomes, summaries))
    }
}
