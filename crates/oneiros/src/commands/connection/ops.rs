use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ConnectionCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(transparent)]
    PrefixResolve(#[from] PrefixError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ConnectionOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateConnectionOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveConnectionOutcomes),
    #[outcome(transparent)]
    List(#[from] ListConnectionsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowConnectionOutcomes),
}

#[derive(Clone, Args)]
pub struct ConnectionOps {
    #[command(subcommand)]
    pub command: ConnectionCommands,
}

impl ConnectionOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ConnectionOutcomes>, ConnectionCommandError> {
        Ok(match &self.command {
            ConnectionCommands::Create(cmd) => cmd.run(context).await?.map_into(),
            ConnectionCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            ConnectionCommands::List(cmd) => cmd.run(context).await?.map_into(),
            ConnectionCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ConnectionCommands {
    /// Create a connection between two links.
    Create(CreateConnection),
    /// Remove a connection.
    Remove(RemoveConnection),
    /// List all connections.
    List(ListConnections),
    /// Show a connection's details.
    Show(ShowConnection),
}
