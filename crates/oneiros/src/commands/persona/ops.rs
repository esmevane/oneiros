use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum PersonaCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum PersonaOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetPersonaOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemovePersonaOutcomes),
    #[outcome(transparent)]
    List(#[from] ListPersonasOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowPersonaOutcomes),
}

#[derive(Clone, Args)]
pub struct PersonaOps {
    #[command(subcommand)]
    pub command: PersonaCommands,
}

impl PersonaOps {
    pub async fn run(
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
pub enum PersonaCommands {
    /// Create or update a persona.
    Set(SetPersona),
    /// Remove a persona.
    Remove(RemovePersona),
    /// List all personas.
    List(ListPersonas),
    /// Show a persona's details.
    Show(ShowPersona),
}
