use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum EventCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
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
    ) -> Result<(Outcomes<EventOutcomes>, Vec<PressureSummary>), EventCommandError> {
        Ok(match &self.command {
            EventCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            EventCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum EventCommands {
    List(ListEvents),
    Show(ShowEvent),
}
