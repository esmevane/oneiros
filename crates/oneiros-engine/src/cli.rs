//! CLI layer collector — assembles domain commands into a unified command tree.
//!
//! Each domain owns its command definitions and execution in `features/cli.rs`.
//! This module collects them into a single top-level enum and routes each
//! command to the context it needs.

use clap::{Parser, Subcommand};

use crate::*;

/// Top-level CLI entry point for the engine.
///
/// Carries global flags (via [`CliOverrides`]) and delegates to
/// [`Command`] for dispatch. No binary entrypoint required — tests
/// and future binaries construct this directly.
#[derive(Debug, Parser)]
#[command(name = "oneiros", version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    #[command(flatten)]
    pub(crate) overrides: CliOverrides,
}

impl Cli {
    /// Execute the command and return the rendered result.
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        self.command.execute(config).await
    }
}

/// All CLI commands, unified under one tree.
///
/// Each command variant knows which context it needs. The caller provides
/// an `Engine` that holds both host and project contexts, and the
/// dispatch routes to the right one.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(subcommand)]
    Host(HostCommands),
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
    Ticket(TicketCommands),
    #[command(subcommand)]
    Peer(PeerCommands),

    /// Manage bookmarks
    #[command(subcommand)]
    Bookmark(BookmarkCommands),

    /// Inspect follow records — bookmark/source links
    #[command(subcommand)]
    Follow(FollowCommands),

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

    /// Walk the events ↔ entities bridge
    #[command(subcommand)]
    Trail(TrailCommands),

    /// Run diagnostics against the local host
    Doctor,
}

impl Command {
    /// Whether this command runs the long-lived HTTP server in-process.
    ///
    /// Only `host run` boots the server here; every other command is a
    /// short-lived CLI client that issues HTTP requests to a running
    /// service. The two surfaces want different tracing defaults — see
    /// `support::logging::level`.
    pub(crate) fn is_server(&self) -> bool {
        matches!(self, Command::Host(HostCommands::Run))
    }

    /// Execute a CLI command against the engine.
    ///
    /// Returns `Rendered` — the caller decides how to consume it:
    /// - `.response()` for typed data access (tests, programmatic use)
    /// - Match on variant for presentation (Prompt content, Text summary)
    /// - `Rendered::Data` is the default for domains without a presenter
    #[tracing::instrument(skip_all, err(Display))]
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        Ok(match self {
            Command::Actor(actor) => actor.execute(config).await?,
            Command::Agent(agent) => agent.execute(config).await?,
            Command::Bookmark(bookmark) => bookmark.execute(config).await?,
            Command::Follow(follow) => follow.execute(config).await?,
            Command::Cognition(cognition) => cognition.execute(config).await?,
            Command::Connection(connection) => connection.execute(config).await?,
            Command::Continuity(continuity) => continuity.execute(config).await?,
            Command::Doctor => DoctorCli::execute(config).await?,
            Command::Experience(experience) => experience.execute(config).await?,
            Command::Level(level) => level.execute(config).await?,
            Command::Mcp(mcp) => mcp.execute(config)?,
            Command::Memory(memory) => memory.execute(config).await?,
            Command::Nature(nature) => nature.execute(config).await?,
            Command::Peer(peer) => peer.execute(config).await?,
            Command::Persona(persona) => persona.execute(config).await?,
            Command::Pressure(pressure) => pressure.execute(config).await?,
            Command::Project(project) => project.execute(config).await?,
            Command::Search(search) => search.execute(config).await?,
            Command::Seed(seed) => seed.execute(config).await?,
            Command::Sensation(sensation) => sensation.execute(config).await?,
            Command::Setup(setup) => SetupCli::execute(config, setup).await?,
            Command::Storage(storage) => storage.execute(config).await?,
            Command::Host(host) => host.execute(config).await?,
            Command::Tenant(tenant) => tenant.execute(config).await?,
            Command::Texture(texture) => texture.execute(config).await?,
            Command::Ticket(ticket) => ticket.execute(config).await?,
            Command::Trail(trail) => trail.execute(config).await?,
            Command::Urge(urge) => urge.execute(config).await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_run_is_the_server() {
        let command = Command::Host(HostCommands::Run);
        assert!(command.is_server());
    }

    #[test]
    fn other_host_subcommands_are_clients() {
        for command in [
            Command::Host(HostCommands::Install),
            Command::Host(HostCommands::Uninstall),
            Command::Host(HostCommands::Start),
            Command::Host(HostCommands::Stop),
            Command::Host(HostCommands::Status),
        ] {
            assert!(!command.is_server(), "{command:?} should be a CLI client");
        }
    }

    #[test]
    fn doctor_is_a_client() {
        assert!(!Command::Doctor.is_server());
    }
}
