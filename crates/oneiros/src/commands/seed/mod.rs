mod core;

use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SeedCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SeedOutcomes {
    #[outcome(transparent)]
    Texture(#[from] SetTextureOutcomes),
    #[outcome(transparent)]
    Level(#[from] SetLevelOutcomes),
    #[outcome(transparent)]
    Persona(#[from] SetPersonaOutcomes),
    #[outcome(transparent)]
    Agent(#[from] CreateAgentOutcomes),
    #[outcome(transparent)]
    Sensation(#[from] SetSensationOutcomes),
    #[outcome(transparent)]
    Nature(#[from] SetNatureOutcomes),
    #[outcome(transparent)]
    Core(#[from] CoreSeedOutcomes),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CoreSeedOutcomes {
    #[outcome(message("Seed failed for {0} '{1}': {2}"), level = "warn")]
    SeedFailed(String, String, String),
    #[outcome(message("Core seed complete."))]
    SeedComplete,
}

impl CoreSeedOutcomes {
    pub fn failed(
        kind: impl AsRef<str>,
        name: impl AsRef<str>,
        error: impl ::core::error::Error,
    ) -> Self {
        Self::SeedFailed(
            kind.as_ref().to_string(),
            name.as_ref().to_string(),
            error.to_string(),
        )
    }
}

/// Apply predefined seed data.
#[derive(Clone, Args)]
pub struct SeedOps {
    #[command(subcommand)]
    command: SeedCommands,
}

#[derive(Clone, Subcommand)]
pub enum SeedCommands {
    /// Seed core textures, levels, personas, sensations, natures, and process agents.
    Core,
}

impl SeedOps {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<SeedOutcomes>, SeedCommandError> {
        match &self.command {
            SeedCommands::Core => run_core(context).await,
        }
    }
}

async fn run_core(context: &Context) -> Result<Outcomes<SeedOutcomes>, SeedCommandError> {
    let mut outcomes = Outcomes::new();
    let mut failures = Outcomes::new();

    for command in core::textures() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("texture", command.name, error)),
        }
    }

    for command in core::levels() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("level", command.name, error)),
        }
    }

    for command in core::personas() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("persona", command.name, error)),
        }
    }

    for command in core::sensations() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("sensation", command.name, error)),
        }
    }

    for command in core::natures() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("nature", command.name, error)),
        }
    }

    for command in core::agents() {
        match command.run(context).await {
            Ok(inner) => outcomes.absorb(inner),
            Err(error) => failures.emit(CoreSeedOutcomes::failed("agent", command.name, error)),
        }
    }

    outcomes.emit(CoreSeedOutcomes::SeedComplete.into());

    Ok(outcomes)
}
