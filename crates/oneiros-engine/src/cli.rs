//! CLI layer collector — assembles domain commands into a unified command tree.
//!
//! Each domain owns its command definitions and execution in `features/cli.rs`.
//! This module collects them into top-level enums and delegates execution.

use clap::Subcommand;

use crate::*;

/// Project-scoped CLI commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
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
    #[command(subcommand)]
    Lifecycle(LifecycleCommands),
    #[command(subcommand)]
    Search(SearchCommands),
    #[command(subcommand)]
    Pressure(PressureCommands),
}

/// System-scoped CLI commands.
#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    #[command(subcommand)]
    Tenant(TenantCommands),
    #[command(subcommand)]
    Actor(ActorCommands),
    #[command(subcommand)]
    Brain(BrainCommands),
    #[command(subcommand)]
    Ticket(TicketCommands),
}

/// Execute a project-scoped command.
pub fn execute(
    ctx: &ProjectContext,
    command: Commands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        Commands::Level(cmd) => LevelCli::execute(ctx, cmd),
        Commands::Texture(cmd) => TextureCli::execute(ctx, cmd),
        Commands::Sensation(cmd) => SensationCli::execute(ctx, cmd),
        Commands::Nature(cmd) => NatureCli::execute(ctx, cmd),
        Commands::Persona(cmd) => PersonaCli::execute(ctx, cmd),
        Commands::Urge(cmd) => UrgeCli::execute(ctx, cmd),
        Commands::Agent(cmd) => AgentCli::execute(ctx, cmd),
        Commands::Cognition(cmd) => CognitionCli::execute(ctx, cmd),
        Commands::Memory(cmd) => MemoryCli::execute(ctx, cmd),
        Commands::Experience(cmd) => ExperienceCli::execute(ctx, cmd),
        Commands::Connection(cmd) => ConnectionCli::execute(ctx, cmd),
        Commands::Lifecycle(cmd) => LifecycleCli::execute(ctx, cmd),
        Commands::Search(cmd) => SearchCli::execute(ctx, cmd),
        Commands::Pressure(cmd) => PressureCli::execute(ctx, cmd),
    }
}

/// Execute a system-scoped command.
pub fn execute_system(
    ctx: &SystemContext,
    command: SystemCommands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        SystemCommands::Tenant(cmd) => TenantCli::execute(ctx, cmd),
        SystemCommands::Actor(cmd) => ActorCli::execute(ctx, cmd),
        SystemCommands::Brain(cmd) => BrainCli::execute(ctx, cmd),
        SystemCommands::Ticket(cmd) => TicketCli::execute(ctx, cmd),
    }
}
