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
    /// Output format: json (default), text, or prompt.
    #[arg(long, short, default_value = "json", global = true)]
    pub output: OutputMode,
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    /// Execute the command and return the rendered result.
    pub async fn execute(&self, engine: &Engine) -> Result<Rendered<Responses>, Error> {
        self.command.execute(engine).await
    }

    /// The selected output mode.
    pub fn output_mode(&self) -> &OutputMode {
        &self.output
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

    // System-scoped domains
    #[command(subcommand)]
    Tenant(TenantCommands),
    #[command(subcommand)]
    Actor(ActorCommands),
    #[command(subcommand)]
    Brain(BrainCommands),
    #[command(subcommand)]
    Ticket(TicketCommands),

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

    // Status — summary of an agent's current cognitive state
    Status {
        name: AgentName,
    },

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
    pub async fn execute(&self, engine: &Engine) -> Result<Rendered<Responses>, Error> {
        Ok(match self {
            // Workflow domains — each knows its context
            Command::System(system) => system.execute(engine.system()).await?,
            Command::Project(project) => {
                project
                    .execute(
                        engine.system(),
                        engine.project.as_ref(),
                        engine.brain_name(),
                    )
                    .await?
            }
            Command::Seed(seed) => seed.execute(engine.project()?).await?,

            // System-scoped domains
            Command::Tenant(tenant) => tenant.execute(engine.system()).await?,
            Command::Actor(actor) => actor.execute(engine.system()).await?,
            Command::Brain(brain) => brain.execute(engine.system()).await?,
            Command::Ticket(ticket) => ticket.execute(engine.system()).await?,

            // Project-scoped domains — vocabulary
            Command::Level(level) => level.execute(engine.project()?).await?,
            Command::Texture(texture) => texture.execute(engine.project()?).await?,
            Command::Sensation(sensation) => sensation.execute(engine.project()?).await?,
            Command::Nature(nature) => nature.execute(engine.project()?).await?,
            Command::Persona(persona) => persona.execute(engine.project()?).await?,
            Command::Urge(urge) => urge.execute(engine.project()?).await?,
            Command::Agent(agent) => agent.execute(engine.project()?).await?,

            // Entity domains — return Rendered with ref_token prompts on create
            Command::Cognition(cognition) => cognition.execute(engine.project()?).await?,
            Command::Memory(memory) => memory.execute(engine.project()?).await?,
            Command::Experience(experience) => experience.execute(engine.project()?).await?,
            Command::Connection(connection) => connection.execute(engine.project()?).await?,

            Command::Storage(storage) => storage.execute(engine.project()?).await?,
            Command::Search(search) => search.execute(engine.project()?).await?,
            Command::Pressure(pressure) => pressure.execute(engine.project()?).await?,

            // Doctor — system diagnostics
            Command::Doctor => DoctorCli::execute(engine.system()).await?,

            // Continuity — domain subcommands go through the presenter
            Command::Continuity(continuity) => continuity.execute(engine.project()?).await?,

            // Flat lifecycle shortcuts — delegate to ContinuityCommands
            Command::Wake { name } => {
                ContinuityCommands::Wake {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Dream { name } => {
                ContinuityCommands::Dream {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Introspect { name } => {
                ContinuityCommands::Introspect {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Reflect { name } => {
                ContinuityCommands::Reflect {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Sleep { name } => {
                ContinuityCommands::Sleep {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Guidebook { name } => {
                ContinuityCommands::Guidebook {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }

            // Continuity lifecycle — emerge, recede, status
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                ContinuityCommands::Emerge {
                    name: name.clone(),
                    persona: persona.clone(),
                    description: description.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Recede { name } => {
                ContinuityCommands::Recede {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
            Command::Status { name } => {
                ContinuityCommands::Status {
                    agent: name.clone(),
                }
                .execute(engine.project()?)
                .await?
            }
        })
    }
}
