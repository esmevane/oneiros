use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum AgentCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
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
    ) -> Result<Outcomes<AgentOutcomes>, AgentCommandError> {
        Ok(match &self.command {
            AgentCommands::Create(cmd) => cmd.run(context).await?.map_into(),
            AgentCommands::Update(cmd) => cmd.run(context).await?.map_into(),
            AgentCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            AgentCommands::List(cmd) => cmd.run(context).await?.map_into(),
            AgentCommands::Show(cmd) => cmd.run(context).await?.map_into(),
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
