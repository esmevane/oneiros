mod outcomes;

mod error;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::PersonaCommandError;
pub(crate) use list::{ListPersonas, ListPersonasOutcomes};
pub(crate) use outcomes::PersonaOutcomes;
pub(crate) use remove::{RemovePersona, RemovePersonaOutcomes};
pub(crate) use set::{SetPersona, SetPersonaOutcomes};
pub(crate) use show::{ShowPersona, ShowPersonaOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct PersonaOps {
    #[command(subcommand)]
    pub command: PersonaCommands,
}

impl PersonaOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<PersonaOutcomes>, PersonaCommandError> {
        Ok(match &self.command {
            PersonaCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            PersonaCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            PersonaCommands::List(cmd) => cmd.run(context).await?.map_into(),
            PersonaCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum PersonaCommands {
    /// Create or update a persona.
    Set(SetPersona),
    /// Remove a persona.
    Remove(RemovePersona),
    /// List all personas.
    List(ListPersonas),
    /// Show a persona's details.
    Show(ShowPersona),
}
