mod outcomes;

mod add;
mod error;
mod list;
mod show;

pub(crate) use add::{AddCognition, AddCognitionOutcomes};
pub(crate) use error::CognitionCommandError;
pub(crate) use list::{ListCognitions, ListCognitionsOutcomes};
pub(crate) use outcomes::CognitionOutcomes;
pub(crate) use show::{ShowCognition, ShowCognitionOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct CognitionOps {
    #[command(subcommand)]
    pub command: CognitionCommands,
}

impl CognitionOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<CognitionOutcomes>, CognitionCommandError> {
        Ok(match &self.command {
            CognitionCommands::Add(cmd) => cmd.run(context).await?.map_into(),
            CognitionCommands::List(cmd) => cmd.run(context).await?.map_into(),
            CognitionCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum CognitionCommands {
    /// Add a new cognition (thought) for an agent.
    Add(AddCognition),
    /// List cognitions, optionally filtered by agent or texture.
    List(ListCognitions),
    /// Show a cognition's details by ID.
    Show(ShowCognition),
}
