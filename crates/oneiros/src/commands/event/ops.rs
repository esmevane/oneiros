use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum EventCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum EventOutcomes {
    #[outcome(transparent)]
    List(#[from] ListEventsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowEventOutcomes),
}

#[derive(Clone, Args)]
pub struct EventOps {
    #[command(subcommand)]
    pub command: EventCommands,
}

impl EventOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<EventOutcomes>, EventCommandError> {
        Ok(match &self.command {
            EventCommands::List(cmd) => cmd.run(context).await?.map_into(),
            EventCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum EventCommands {
    List(ListEvents),
    Show(ShowEvent),
}
