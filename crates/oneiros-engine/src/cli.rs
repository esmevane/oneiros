//! CLI layer collector — assembles domain commands into a unified command tree.
//!
//! Each domain owns its command definitions and execution in `features/cli.rs`.
//! This module collects them into a single top-level enum and routes each
//! command to the context it needs.

use clap::Subcommand;

use crate::*;

/// All CLI commands, unified under one tree.
///
/// Each command variant knows which context it needs. The caller provides
/// an `EngineContext` that holds both system and project contexts, and the
/// dispatch routes to the right one.
#[derive(Debug, Subcommand)]
pub enum Command {
    // Workflow domains
    #[command(subcommand)]
    System(SystemCommands),
    #[command(subcommand)]
    Project(ProjectCommands),
    #[command(subcommand)]
    Seed(SeedCommands),

    // Vocabulary domains (project-scoped)
    #[command(subcommand)]
    Level(LevelCommands),
    #[command(subcommand)]
    Texture(TextureCommands),
    #[command(subcommand)]
    Sensation(SensationCommands),
    #[command(subcommand)]
    Nature(NatureCommands),
    #[command(subcommand)]
    Persona(PersonaCommands),
    #[command(subcommand)]
    Urge(UrgeCommands),

    // Entity domains (project-scoped)
    #[command(subcommand)]
    Agent(AgentCommands),
    #[command(subcommand)]
    Cognition(CognitionCommands),
    #[command(subcommand)]
    Memory(MemoryCommands),
    #[command(subcommand)]
    Experience(ExperienceCommands),
    #[command(subcommand)]
    Connection(ConnectionCommands),

    // Lifecycle / derived (project-scoped)
    #[command(subcommand)]
    Lifecycle(LifecycleCommands),
    #[command(subcommand)]
    Storage(StorageCommands),

    // Flat arg commands — args appear directly under the command name
    Search(#[command(flatten)] SearchCommands),
    Pressure(#[command(flatten)] PressureCommands),
}

/// The combined context for CLI execution.
///
/// Holds both system and project contexts. System context is always
/// available. Project context is available after `project init` and
/// `start_service`.
pub struct EngineContext {
    pub system: SystemContext,
    pub project: Option<ProjectContext>,
    pub brain_name: String,
}

impl EngineContext {
    pub fn project(&self) -> Result<&ProjectContext, Box<dyn std::error::Error>> {
        self.project
            .as_ref()
            .ok_or_else(|| "project context required — call start_service first".into())
    }
}

/// Execute a CLI command against the engine context.
pub fn execute(
    ctx: &EngineContext,
    command: Command,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        // Workflow domains — each knows its context
        Command::System(cmd) => SystemCli::execute(&ctx.system, cmd),
        Command::Project(cmd) => ProjectCli::execute(&ctx.system, &ctx.brain_name, cmd),
        Command::Seed(cmd) => SeedCli::execute(ctx.project()?, cmd),

        // Project-scoped domains
        Command::Level(cmd) => LevelCli::execute(ctx.project()?, cmd),
        Command::Texture(cmd) => TextureCli::execute(ctx.project()?, cmd),
        Command::Sensation(cmd) => SensationCli::execute(ctx.project()?, cmd),
        Command::Nature(cmd) => NatureCli::execute(ctx.project()?, cmd),
        Command::Persona(cmd) => PersonaCli::execute(ctx.project()?, cmd),
        Command::Urge(cmd) => UrgeCli::execute(ctx.project()?, cmd),
        Command::Agent(cmd) => AgentCli::execute(ctx.project()?, cmd),
        Command::Cognition(cmd) => CognitionCli::execute(ctx.project()?, cmd),
        Command::Memory(cmd) => MemoryCli::execute(ctx.project()?, cmd),
        Command::Experience(cmd) => ExperienceCli::execute(ctx.project()?, cmd),
        Command::Connection(cmd) => ConnectionCli::execute(ctx.project()?, cmd),
        Command::Lifecycle(cmd) => LifecycleCli::execute(ctx.project()?, cmd),
        Command::Storage(cmd) => StorageCli::execute(ctx.project()?, cmd),
        Command::Search(cmd) => SearchCli::execute(ctx.project()?, cmd),
        Command::Pressure(cmd) => PressureCli::execute(ctx.project()?, cmd),
    }
}
