mod error;
mod log_config;
mod outcomes;

use clap::Parser;
use clap::Subcommand;
use oneiros_outcomes::Outcomes;

pub(crate) use error::*;
pub(crate) use log_config::LogConfig;
pub(crate) use outcomes::CliOutcomes;

use crate::*;

#[derive(Clone, Parser)]
#[command(version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) log: LogConfig,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    pub(crate) async fn run(&self) -> Result<Outcomes<CliOutcomes>, CliError> {
        let context = Context::init()?;

        Ok(match &self.command {
            Command::Agent(agent) => agent.run(context).await?.map_into(),
            Command::Doctor(doctor) => doctor.run(context).await?.map_into(),
            Command::Level(level) => level.run(context).await?.map_into(),
            Command::Persona(persona) => persona.run(context).await?.map_into(),
            Command::System(system) => system.run(context).await?.map_into(),
            Command::Service(service) => service.run(context).await?.map_into(),
            Command::Project(project) => project.run(context).await?.map_into(),
            Command::Texture(texture) => texture.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    /// Manage agents (named participants in a brain's cognition).
    Agent(AgentOps),
    /// Check the health of the local oneiros host and the current project.
    Doctor(Doctor),
    /// Manage levels (memory retention tiers).
    Level(LevelOps),
    /// Manage personas (named agent roles).
    Persona(PersonaOps),
    /// Project-level commands (init, etc.).
    Project(ProjectOps),
    /// Manage the oneiros service (run, status).
    Service(ServiceOps),
    /// System-level commands for the local oneiros host (init, status, etc.).
    System(SystemOps),
    /// Manage textures (cognitive categories for agent thoughts).
    Texture(TextureOps),
}
