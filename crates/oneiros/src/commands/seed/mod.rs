mod core;
mod error;
mod outcomes;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

pub(crate) use error::SeedCommandError;
pub(crate) use outcomes::{CoreSeedOutcomes, SeedOutcomes};

use crate::*;

/// Apply predefined seed data.
#[derive(Clone, Args)]
pub(crate) struct SeedOps {
    #[command(subcommand)]
    command: SeedCommands,
}

#[derive(Clone, Subcommand)]
pub(crate) enum SeedCommands {
    /// Seed core textures, levels, personas, sensations, natures, and process agents.
    Core,
}

impl SeedOps {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SeedOutcomes>, SeedCommandError> {
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
