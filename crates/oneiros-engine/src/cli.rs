//! CLI layer collector — assembles domain commands into a unified command tree.
//!
//! Each domain owns its command definitions and execution in `features/cli.rs`.
//! This module collects them into a single top-level enum and routes each
//! command to the context it needs.

use clap::{Parser, Subcommand};

use crate::*;

/// Top-level CLI entry point for the engine.
///
/// Carries the global `--output` flag and delegates to `Command` for dispatch.
/// No binary entrypoint required — tests and future binaries construct this directly.
#[derive(Debug, Parser)]
#[command(name = "oneiros", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[command(flatten)]
    config: Config,
}

impl Cli {
    /// Execute the command and return the rendered result.
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        self.command.execute(config).await
    }

    /// The parsed config (from CLI flags).
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The selected output mode.
    pub fn output_mode(&self) -> &OutputMode {
        &self.config.output
    }
}

/// All CLI commands, unified under one tree.
///
/// Each command variant knows which context it needs. The caller provides
/// an `Engine` that holds both system and project contexts, and the
/// dispatch routes to the right one.
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    System(SystemCommands),
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Seed the given project with presets
    #[command(subcommand)]
    Seed(SeedCommands),
    /// Manage MCP-related things
    #[command(subcommand)]
    Mcp(McpCommands),
    /// Guided first-run setup.
    Setup(#[command(flatten)] SetupRequest),

    #[command(subcommand)]
    Tenant(TenantCommands),
    #[command(subcommand)]
    Actor(ActorCommands),
    #[command(subcommand)]
    Brain(BrainCommands),
    #[command(subcommand)]
    Ticket(TicketCommands),
    #[command(subcommand)]
    Peer(PeerCommands),

    /// Manage bookmarks
    #[command(subcommand)]
    Bookmark(BookmarkCommands),

    /// Install and start the service, or run it directly
    #[command(subcommand)]
    Service(ServiceCommands),

    /// Manage the agents within your continuity
    #[command(subcommand)]
    Agent(AgentCommands),
    /// Record a thought you've had
    #[command(subcommand)]
    Cognition(CognitionCommands),
    /// Track something you want to remember
    #[command(subcommand)]
    Memory(MemoryCommands),
    /// Describe impressions and things that happened to you
    #[command(subcommand)]
    Experience(ExperienceCommands),
    /// Draw links between continuity
    #[command(subcommand)]
    Connection(ConnectionCommands),
    /// Manage levels for your memories
    #[command(subcommand)]
    Level(LevelCommands),
    /// Manage texures for your cognitions
    #[command(subcommand)]
    Texture(TextureCommands),
    /// Manage sensations for your experiences
    #[command(subcommand)]
    Sensation(SensationCommands),
    /// Manage natures for your connections
    #[command(subcommand)]
    Nature(NatureCommands),
    /// Manage personas for your agents
    #[command(subcommand)]
    Persona(PersonaCommands),
    /// Search through continuity
    Search(#[command(flatten)] SearchCommands),
    /// Review current pressure gauges
    Pressure(#[command(flatten)] PressureCommands),
    /// Manage urges for your pressure gauges
    #[command(subcommand)]
    Urge(UrgeCommands),
    /// Store files in an embedded file system
    #[command(subcommand)]
    Storage(StorageCommands),

    /// Continuity lifecycle commands
    #[command(subcommand)]
    Continuity(ContinuityCommands),

    /// Run diagnostics against the local host
    Doctor,
}

impl Command {
    /// Execute a CLI command against the engine.
    ///
    /// Returns `Rendered` — the caller decides how to consume it:
    /// - `.response()` for typed data access (tests, programmatic use)
    /// - Match on variant for presentation (Prompt content, Text summary)
    /// - `Rendered::Data` is the default for domains without a presenter
    #[tracing::instrument(skip_all, err(Display))]
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        let system_ctx = || -> Result<HostLog, Error> {
            Ok(ComposeScope::new(config.clone()).host()?.host_log())
        };

        let project_ctx = || -> Result<ProjectLog, Error> {
            Ok(ComposeScope::new(config.clone())
                .bookmark(config.brain.clone(), config.bookmark.clone())?
                .project_log())
        };

        Ok(match self {
            Command::Actor(actor) => actor.execute(&system_ctx()?).await?,
            Command::Agent(agent) => agent.execute(&project_ctx()?).await?,
            Command::Bookmark(bookmark) => bookmark.execute(&project_ctx()?).await?,
            Command::Brain(brain) => brain.execute(&system_ctx()?).await?,
            Command::Cognition(cognition) => cognition.execute(&project_ctx()?).await?,
            Command::Connection(connection) => connection.execute(&project_ctx()?).await?,
            Command::Continuity(continuity) => continuity.execute(&project_ctx()?).await?,
            Command::Doctor => DoctorCli::execute(config).await?,
            Command::Experience(experience) => experience.execute(&project_ctx()?).await?,
            Command::Level(level) => level.execute(&project_ctx()?).await?,
            Command::Mcp(mcp) => mcp.execute(config)?,
            Command::Memory(memory) => memory.execute(&project_ctx()?).await?,
            Command::Nature(nature) => nature.execute(&project_ctx()?).await?,
            Command::Peer(peer) => peer.execute(&system_ctx()?).await?,
            Command::Persona(persona) => persona.execute(&project_ctx()?).await?,
            Command::Pressure(pressure) => pressure.execute(&project_ctx()?).await?,
            Command::Project(project) => project.execute(config).await?,
            Command::Search(search) => search.execute(&project_ctx()?).await?,
            Command::Seed(seed) => seed.execute(&project_ctx()?).await?,
            Command::Sensation(sensation) => sensation.execute(&project_ctx()?).await?,
            Command::Service(service) => service.execute(config).await?,
            Command::Setup(setup) => SetupCli::execute(config, setup).await?,
            Command::Storage(storage) => storage.execute(&project_ctx()?).await?,
            Command::System(system) => system.execute(system_ctx()?).await?,
            Command::Tenant(tenant) => tenant.execute(&system_ctx()?).await?,
            Command::Texture(texture) => texture.execute(&project_ctx()?).await?,
            Command::Ticket(ticket) => ticket.execute(&system_ctx()?).await?,
            Command::Urge(urge) => urge.execute(&project_ctx()?).await?,
        })
    }
}
