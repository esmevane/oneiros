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
    pub(crate) command: Command,
    #[command(flatten)]
    config: Config,
}

impl Cli {
    /// Execute the command and return the rendered result.
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        self.command.execute(config).await
    }

    /// The parsed config (from CLI flags).
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }
}

/// All CLI commands, unified under one tree.
///
/// Each command variant knows which context it needs. The caller provides
/// an `Engine` that holds both system and project contexts, and the
/// dispatch routes to the right one.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    // Workflow domains
    #[command(subcommand)]
    System(SystemCommands),
    #[command(subcommand)]
    Project(ProjectCommands),
    #[command(subcommand)]
    Seed(SeedCommands),
    #[command(subcommand)]
    Mcp(McpCommands),
    /// Guided first-run setup.
    Setup(#[command(flatten)] SetupRequest),

    // System-scoped domains
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

    // Bookmark — canon navigation
    #[command(subcommand)]
    Bookmark(BookmarkCommands),

    // Service management
    #[command(subcommand)]
    Service(ServiceCommands),

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

    // Continuity / derived (project-scoped)
    #[command(subcommand)]
    Continuity(ContinuityCommands),
    #[command(subcommand)]
    Storage(StorageCommands),

    // Flat arg commands — args appear directly under the command name
    Search(#[command(flatten)] SearchCommands),
    Pressure(#[command(flatten)] PressureCommands),

    // Flat lifecycle commands — top-level shortcuts for the most common ops
    Wake {
        name: AgentName,
        /// Render the full dream with all vocabulary and memories inline.
        #[arg(long)]
        deep: bool,
    },
    Dream {
        name: AgentName,
        /// Render the full dream with all vocabulary and memories inline.
        #[arg(long)]
        deep: bool,
    },
    Introspect {
        name: AgentName,
    },
    Reflect {
        name: AgentName,
    },
    Sense {
        name: AgentName,
        #[arg(default_value = "")]
        content: Content,
    },
    Sleep {
        name: AgentName,
    },
    Guidebook {
        name: AgentName,
    },

    // Agent lifecycle — emerge creates then wakes; recede removes
    Emerge {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
    },
    Recede {
        name: AgentName,
    },

    // Status — cross-agent activity overview
    Status,

    // Diagnostics
    Doctor,
}

impl Command {
    /// Execute a CLI command against the engine.
    ///
    /// Returns `Rendered` — the caller decides how to consume it:
    /// - `.response()` for typed data access (tests, programmatic use)
    /// - Match on variant for presentation (Prompt content, Text summary)
    /// - `Rendered::Data` is the default for domains without a presenter
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        let client = config.client();

        Ok(match self {
            // Service management — operates before/outside HTTP transport
            Command::Service(service) => service.execute(config).await?,

            // Bootstrap — direct context access, no HTTP
            Command::System(system) => system.execute(config.system()).await?,
            Command::Project(project) => project.execute(config).await?,
            Command::Mcp(mcp) => mcp.execute(config)?,
            Command::Setup(setup) => SetupCli::execute(config, setup).await?,

            // All domain commands route through HTTP client
            Command::Seed(seed) => seed.execute(&client).await?,
            Command::Bookmark(bookmark) => bookmark.execute(&client).await?,

            Command::Tenant(tenant) => tenant.execute(&client).await?,
            Command::Actor(actor) => actor.execute(&client).await?,
            Command::Brain(brain) => brain.execute(&client).await?,
            Command::Ticket(ticket) => ticket.execute(&client).await?,
            Command::Peer(peer) => peer.execute(&client).await?,

            Command::Level(level) => level.execute(&client).await?,
            Command::Texture(texture) => texture.execute(&client).await?,
            Command::Sensation(sensation) => sensation.execute(&client).await?,
            Command::Nature(nature) => nature.execute(&client).await?,
            Command::Persona(persona) => persona.execute(&client).await?,
            Command::Urge(urge) => urge.execute(&client).await?,
            Command::Agent(agent) => agent.execute(&client).await?,

            Command::Cognition(cognition) => cognition.execute(&client).await?,
            Command::Memory(memory) => memory.execute(&client).await?,
            Command::Experience(experience) => experience.execute(&client).await?,
            Command::Connection(connection) => connection.execute(&client).await?,

            Command::Storage(storage) => storage.execute(&client).await?,
            Command::Search(search) => search.execute(&client).await?,
            Command::Pressure(pressure) => pressure.execute(&client).await?,

            Command::Doctor => DoctorCli::execute(config).await?,

            Command::Continuity(continuity) => continuity.execute(&client).await?,

            Command::Wake { name, deep } => {
                ContinuityCommands::Wake(WakeAgent {
                    agent: name.clone(),
                    deep: *deep,
                })
                .execute(&client)
                .await?
            }
            Command::Dream { name, deep } => {
                ContinuityCommands::Dream(DreamAgent {
                    agent: name.clone(),
                    deep: *deep,
                })
                .execute(&client)
                .await?
            }
            Command::Introspect { name } => {
                ContinuityCommands::Introspect(IntrospectAgent {
                    agent: name.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Reflect { name } => {
                ContinuityCommands::Reflect(ReflectAgent {
                    agent: name.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Sense { name, content } => {
                ContinuityCommands::Sense(SenseContent {
                    agent: name.clone(),
                    content: content.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Sleep { name } => {
                ContinuityCommands::Sleep(SleepAgent {
                    agent: name.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Guidebook { name } => {
                ContinuityCommands::Guidebook(GuidebookAgent {
                    agent: name.clone(),
                })
                .execute(&client)
                .await?
            }

            Command::Emerge {
                name,
                persona,
                description,
            } => {
                ContinuityCommands::Emerge(EmergeAgent {
                    name: name.clone(),
                    persona: persona.clone(),
                    description: description.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Recede { name } => {
                ContinuityCommands::Recede(RecedeAgent {
                    agent: name.clone(),
                })
                .execute(&client)
                .await?
            }
            Command::Status => {
                ContinuityCommands::Status(StatusAgent::default())
                    .execute(&client)
                    .await?
            }
        })
    }
}
