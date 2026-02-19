mod outcomes;

mod create;
mod error;
mod list;
mod remove;
mod show;

pub(crate) use create::{CreateConnection, CreateConnectionOutcomes};
pub(crate) use error::ConnectionCommandError;
pub(crate) use list::{ListConnections, ListConnectionsOutcomes};
pub(crate) use outcomes::ConnectionOutcomes;
pub(crate) use remove::{RemoveConnection, RemoveConnectionOutcomes};
pub(crate) use show::{ShowConnection, ShowConnectionOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct ConnectionOps {
    #[command(subcommand)]
    pub command: ConnectionCommands,
}

impl ConnectionOps {
    pub(crate) async fn run(
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
pub(crate) enum ConnectionCommands {
    /// Create a connection between two links.
    Create(CreateConnection),
    /// Remove a connection.
    Remove(RemoveConnection),
    /// List all connections.
    List(ListConnections),
    /// Show a connection's details.
    Show(ShowConnection),
}
