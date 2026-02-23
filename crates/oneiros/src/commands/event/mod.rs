mod error;
mod list;
mod show;

pub(crate) use error::EventCommandError;
pub(crate) use list::{ListEvents, ListEventsOutcomes};
pub(crate) use show::{ShowEvent, ShowEventOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum EventOutcomes {
    #[outcome(transparent)]
    List(#[from] ListEventsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowEventOutcomes),
}

#[derive(Clone, Args)]
pub(crate) struct EventOps {
    #[command(subcommand)]
    pub command: EventCommands,
}

impl EventOps {
    pub(crate) async fn run(
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
pub(crate) enum EventCommands {
    List(ListEvents),
    Show(ShowEvent),
}
