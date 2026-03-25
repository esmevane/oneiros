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

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
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
    ) -> Result<
        (
            Outcomes<ConnectionOutcomes>,
            Vec<PressureSummary>,
            Option<RefToken>,
        ),
        ConnectionCommandError,
    > {
        Ok(match &self.command {
            ConnectionCommands::Create(cmd) => {
                let (o, s, r) = cmd.run(context).await?;
                (o.map_into(), s, r)
            }
            ConnectionCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s, None)
            }
            ConnectionCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s, None)
            }
            ConnectionCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s, None)
            }
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
