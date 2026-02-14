mod outcomes;

mod create;
mod error;
mod list;
mod remove;
mod show;
mod update;

pub(crate) use create::{CreateAgent, CreateAgentOutcomes};
pub(crate) use error::AgentCommandError;
pub(crate) use list::{ListAgents, ListAgentsOutcomes};
pub(crate) use outcomes::AgentOutcomes;
pub(crate) use remove::{RemoveAgent, RemoveAgentOutcomes};
pub(crate) use show::{ShowAgent, ShowAgentOutcomes};
pub(crate) use update::{UpdateAgent, UpdateAgentOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct AgentOps {
    #[command(subcommand)]
    pub command: AgentCommands,
}

impl AgentOps {
    pub(crate) async fn run(
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
pub(crate) enum AgentCommands {
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
