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

    // Lifecycle / derived (project-scoped)
    #[command(subcommand)]
    Lifecycle(LifecycleCommands),
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
    pub fn execute(&self, engine: &Engine) -> Result<Response<Responses>, Error> {
        Ok(match self {
            // Workflow domains — each knows its context
            Command::System(system) => Response::new(system.execute(engine.system())?),
            Command::Project(project) => Response::new(
                project
                    .execute(
                        engine.system(),
                        engine.project.as_ref(),
                        engine.brain_name(),
                    )
                    .map_err(|e| Error::Context(e.to_string()))?,
            ),
            Command::Seed(seed) => Response::new(
                seed.execute(engine.project()?)
                    .map_err(|e| Error::Context(e.to_string()))?,
            ),

            // Project-scoped domains — vocabulary (no ref_token)
            Command::Level(level) => Response::new(level.execute(engine.project()?)?),
            Command::Texture(texture) => Response::new(texture.execute(engine.project()?)?),
            Command::Sensation(sensation) => Response::new(sensation.execute(engine.project()?)?),
            Command::Nature(nature) => Response::new(nature.execute(engine.project()?)?),
            Command::Persona(persona) => Response::new(persona.execute(engine.project()?)?),
            Command::Urge(urge) => Response::new(urge.execute(engine.project()?)?),
            Command::Agent(agent) => Response::new(agent.execute(engine.project()?)?),

            // Entity domains — return Response<Responses> with ref_token in meta
            Command::Cognition(cognition) => cognition.execute(engine.project()?)?,
            Command::Memory(memory) => memory.execute(engine.project()?)?,
            Command::Experience(experience) => experience.execute(engine.project()?)?,
            Command::Connection(connection) => connection.execute(engine.project()?)?,

            Command::Storage(storage) => Response::new(storage.execute(engine.project()?)?),
            Command::Search(search) => Response::new(search.execute(engine.project()?)?),
            Command::Pressure(pressure) => Response::new(pressure.execute(engine.project()?)?),

            // Doctor — system diagnostics
            Command::Doctor => Response::new(
                DoctorCli::execute(engine.system()).map_err(|e| Error::Context(e.to_string()))?,
            ),

            // Lifecycle
            Command::Lifecycle(lifecycle) => Response::new(lifecycle.execute(engine.project()?)?),
            Command::Wake { name } => {
                Response::new(LifecycleService::wake(engine.project()?, &name)?.into())
            }
            Command::Dream { name } => {
                Response::new(LifecycleService::dream(engine.project()?, &name)?.into())
            }
            Command::Introspect { name } => {
                Response::new(LifecycleService::introspect(engine.project()?, &name)?.into())
            }
            Command::Reflect { name } => {
                Response::new(LifecycleService::reflect(engine.project()?, &name)?.into())
            }
            Command::Sleep { name } => {
                Response::new(LifecycleService::sleep(engine.project()?, &name)?.into())
            }
            Command::Guidebook { name } => {
                Response::new(LifecycleService::guidebook(engine.project()?, &name)?.into())
            }

            // Emerge: create an agent then immediately wake it
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                let project = engine.project()?;
                let created = AgentService::create(
                    project,
                    name.clone(),
                    persona.clone(),
                    description.clone(),
                    Prompt::new(""),
                )?;
                let agent_name = match &created {
                    AgentResponse::AgentCreated(n) => n.clone(),
                    other => {
                        return Err(Error::Context(format!(
                            "unexpected agent response: {other:?}"
                        )));
                    }
                };
                LifecycleService::wake(project, &agent_name)?;
                Response::new(
                    serde_json::json!({ "type": "emerged", "data": agent_name.to_string() }).into(),
                )
            }

            // Recede: retire an agent
            Command::Recede { name } => {
                let project = engine.project()?;
                let name_str = name.to_string();
                AgentService::remove(project, &name)?;
                Response::new(serde_json::json!({ "type": "receded", "data": name_str }).into())
            }

            // Status: gather an agent's cognitive context and return it
            Command::Status { name } => {
                let project = engine.project()?;
                let context = LifecycleService::gather_context(project, &name)?;
                Response::new(serde_json::json!({ "type": "status", "data": context }).into())
            }
        })
    }
}
