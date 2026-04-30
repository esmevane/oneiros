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
    },
    Dream {
        name: AgentName,
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
    #[tracing::instrument(skip_all, err(Display))]
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, Error> {
        // Strangler bridges — produce legacy Context types via Scope.
        // Will simplify when consumers migrate to take Scope directly.
        let system_ctx = || -> Result<HostLog, Error> {
            Ok(ComposeScope::new(config.clone()).host()?.host_log())
        };
        let project_ctx = || -> Result<ProjectLog, Error> {
            Ok(ComposeScope::new(config.clone())
                .bookmark(config.brain.clone(), config.bookmark.clone())?
                .project_log())
        };

        Ok(match self {
            // Service management — operates before/outside HTTP transport
            Command::Service(service) => service.execute(config).await?,

            // Workflow domains — each knows its context
            Command::System(system) => system.execute(system_ctx()?).await?,
            Command::Project(project) => project.execute(config).await?,
            Command::Seed(seed) => seed.execute(&project_ctx()?).await?,
            Command::Mcp(mcp) => mcp.execute(config)?,
            Command::Setup(setup) => SetupCli::execute(config, setup).await?,

            // Bookmark — canon navigation (routes through HTTP)
            Command::Bookmark(bookmark) => bookmark.execute(&project_ctx()?).await?,

            // System-scoped domains
            Command::Tenant(tenant) => tenant.execute(&system_ctx()?).await?,
            Command::Actor(actor) => actor.execute(&system_ctx()?).await?,
            Command::Brain(brain) => brain.execute(&system_ctx()?).await?,
            Command::Ticket(ticket) => ticket.execute(&system_ctx()?).await?,
            Command::Peer(peer) => peer.execute(&system_ctx()?).await?,

            // Project-scoped domains — vocabulary
            Command::Level(level) => level.execute(&project_ctx()?).await?,
            Command::Texture(texture) => texture.execute(&project_ctx()?).await?,
            Command::Sensation(sensation) => sensation.execute(&project_ctx()?).await?,
            Command::Nature(nature) => nature.execute(&project_ctx()?).await?,
            Command::Persona(persona) => persona.execute(&project_ctx()?).await?,
            Command::Urge(urge) => urge.execute(&project_ctx()?).await?,
            Command::Agent(agent) => agent.execute(&project_ctx()?).await?,

            // Entity domains — return Rendered with ref_token prompts on create
            Command::Cognition(cognition) => cognition.execute(&project_ctx()?).await?,
            Command::Memory(memory) => memory.execute(&project_ctx()?).await?,
            Command::Experience(experience) => experience.execute(&project_ctx()?).await?,
            Command::Connection(connection) => connection.execute(&project_ctx()?).await?,

            Command::Storage(storage) => storage.execute(&project_ctx()?).await?,
            Command::Search(search) => search.execute(&project_ctx()?).await?,
            Command::Pressure(pressure) => pressure.execute(&project_ctx()?).await?,

            // Doctor — system diagnostics
            Command::Doctor => DoctorCli::execute(config).await?,

            // Continuity — domain subcommands go through the presenter
            Command::Continuity(continuity) => continuity.execute(&project_ctx()?).await?,

            // Flat lifecycle shortcuts — delegate to ContinuityCommands
            Command::Wake { name } => {
                ContinuityCommands::Wake(WakeAgent::builder_v1().agent(name.clone()).build().into())
                    .execute(&project_ctx()?)
                    .await?
            }
            Command::Dream { name } => {
                ContinuityCommands::Dream(
                    DreamAgent::builder_v1().agent(name.clone()).build().into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Introspect { name } => {
                ContinuityCommands::Introspect(
                    IntrospectAgent::builder_v1()
                        .agent(name.clone())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Reflect { name } => {
                ContinuityCommands::Reflect(
                    ReflectAgent::builder_v1()
                        .agent(name.clone())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Sense { name, content } => {
                ContinuityCommands::Sense(
                    SenseContent::builder_v1()
                        .agent(name.clone())
                        .content(content.clone())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Sleep { name } => {
                ContinuityCommands::Sleep(
                    SleepAgent::builder_v1().agent(name.clone()).build().into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Guidebook { name } => {
                ContinuityCommands::Guidebook(
                    GuidebookAgent::builder_v1()
                        .agent(name.clone())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }

            // Continuity lifecycle — emerge, recede, status
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                ContinuityCommands::Emerge(
                    EmergeAgent::builder_v1()
                        .name(name.clone())
                        .persona(persona.clone())
                        .description(description.clone())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Recede { name } => {
                ContinuityCommands::Recede(
                    RecedeAgent::builder_v1().agent(name.clone()).build().into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
            Command::Status => {
                ContinuityCommands::Status(
                    StatusAgent::builder_v1()
                        .filters(SearchFilters::default())
                        .build()
                        .into(),
                )
                .execute(&project_ctx()?)
                .await?
            }
        })
    }
}
