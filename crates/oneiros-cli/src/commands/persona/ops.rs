use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum PersonaCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
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
    ) -> Result<(Outcomes<PersonaOutcomes>, Vec<PressureSummary>), PersonaCommandError> {
        Ok(match &self.command {
            PersonaCommands::Set(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            PersonaCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            PersonaCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            PersonaCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
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
