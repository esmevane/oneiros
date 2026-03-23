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
            Command::System(system) => Response::new(system.execute(engine.system())?).into(),
            Command::Project(project) => Response::new(
                project
                    .execute(
                        engine.system(),
                        engine.project.as_ref(),
                        engine.brain_name(),
                    )
                    .map_err(|e| Error::Context(e.to_string()))?,
            )
            .into(),
            Command::Seed(seed) => Response::new(
                seed.execute(engine.project()?)
                    .map_err(|e| Error::Context(e.to_string()))?,
            )
            .into(),

            // Project-scoped domains — vocabulary (no ref_token)
            Command::Level(level) => Response::new(level.execute(engine.project()?).await?).into(),
            Command::Texture(texture) => {
                Response::new(texture.execute(engine.project()?).await?).into()
            }
            Command::Sensation(sensation) => {
                Response::new(sensation.execute(engine.project()?).await?).into()
            }
            Command::Nature(nature) => {
                Response::new(nature.execute(engine.project()?).await?).into()
            }
            Command::Persona(persona) => {
                Response::new(persona.execute(engine.project()?).await?).into()
            }
            Command::Urge(urge) => Response::new(urge.execute(engine.project()?).await?).into(),
            Command::Agent(agent) => Response::new(agent.execute(engine.project()?).await?).into(),

            // Entity domains — return Response<Responses> with ref_token in meta
            Command::Cognition(cognition) => cognition.execute(engine.project()?).await?.into(),
            Command::Memory(memory) => memory.execute(engine.project()?).await?.into(),
            Command::Experience(experience) => experience.execute(engine.project()?).await?.into(),
            Command::Connection(connection) => connection.execute(engine.project()?).await?.into(),

            Command::Storage(storage) => {
                Response::new(storage.execute(engine.project()?).await?).into()
            }
            Command::Search(search) => {
                Response::new(search.execute(engine.project()?).await?).into()
            }
            Command::Pressure(pressure) => {
                Response::new(pressure.execute(engine.project()?).await?).into()
            }

            // Doctor — system diagnostics
            Command::Doctor => Response::new(
                DoctorCli::execute(engine.system()).map_err(|e| Error::Context(e.to_string()))?,
            )
            .into(),

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
