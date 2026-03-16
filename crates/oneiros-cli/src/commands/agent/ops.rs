use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum AgentCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum AgentOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateAgentOutcomes),
    #[outcome(transparent)]
    Update(#[from] UpdateAgentOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveAgentOutcomes),
    #[outcome(transparent)]
    List(#[from] ListAgentsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowAgentOutcomes),
}

#[derive(Clone, Args)]
pub struct AgentOps {
    #[command(subcommand)]
    pub command: AgentCommands,
}

impl AgentOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<(Outcomes<AgentOutcomes>, Vec<PressureSummary>), AgentCommandError> {
        Ok(match &self.command {
            AgentCommands::Create(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            AgentCommands::Update(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            AgentCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            AgentCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            AgentCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum AgentCommands {
    /// Create a new agent.
    Create(CreateAgent),
    /// Update an existing agent.
    Update(UpdateAgent),
    /// Remove an agent.
    Remove(RemoveAgent),
    /// List all agents.
    List(ListAgents),
    /// Show an agent's details.
    Show(ShowAgent),
}
