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
    /// Execute a CLI command against the engine context.
    pub fn execute(&self, context: &EngineContext) -> Result<Response<Responses>, Error> {
        Ok(match self {
            // Workflow domains — each knows its context
            Command::System(system) => Response::new(system.execute(&context.system)?),
            Command::Project(project) => Response::new(
                project
                    .execute(
                        &context.system,
                        context.project.as_ref(),
                        &context.brain_name,
                    )
                    .map_err(|e| Error::Context(e.to_string()))?,
            ),
            Command::Seed(seed) => Response::new(
                seed.execute(context.project()?)
                    .map_err(|e| Error::Context(e.to_string()))?,
            ),

            // Project-scoped domains — vocabulary (no ref_token)
            Command::Level(level) => Response::new(level.execute(context.project()?)?),
            Command::Texture(texture) => Response::new(texture.execute(context.project()?)?),
            Command::Sensation(sensation) => {
                Response::new(sensation.execute(context.project()?)?)
            }
            Command::Nature(nature) => Response::new(nature.execute(context.project()?)?),
            Command::Persona(persona) => Response::new(persona.execute(context.project()?)?),
            Command::Urge(urge) => Response::new(urge.execute(context.project()?)?),
            Command::Agent(agent) => Response::new(agent.execute(context.project()?)?),

            // Entity domains — return Response<Responses> with ref_token in meta
            Command::Cognition(cognition) => cognition.execute(context.project()?)?,
            Command::Memory(memory) => memory.execute(context.project()?)?,
            Command::Experience(experience) => experience.execute(context.project()?)?,
            Command::Connection(connection) => connection.execute(context.project()?)?,

            Command::Storage(storage) => Response::new(storage.execute(context.project()?)?),
            Command::Search(search) => Response::new(search.execute(context.project()?)?),
            Command::Pressure(pressure) => Response::new(pressure.execute(context.project()?)?),

            // Doctor — system diagnostics
            Command::Doctor => Response::new(
                DoctorCli::execute(&context.system).map_err(|e| Error::Context(e.to_string()))?,
            ),

            // Lifecycle
            Command::Lifecycle(lifecycle) => {
                Response::new(lifecycle.execute(context.project()?)?)
            }
            Command::Wake { name } => {
                Response::new(LifecycleService::wake(context.project()?, &name)?.into())
            }
            Command::Dream { name } => {
                Response::new(LifecycleService::dream(context.project()?, &name)?.into())
            }
            Command::Introspect { name } => {
                Response::new(LifecycleService::introspect(context.project()?, &name)?.into())
            }
            Command::Reflect { name } => {
                Response::new(LifecycleService::reflect(context.project()?, &name)?.into())
            }
            Command::Sleep { name } => {
                Response::new(LifecycleService::sleep(context.project()?, &name)?.into())
            }
            Command::Guidebook { name } => {
                Response::new(LifecycleService::guidebook(context.project()?, &name)?.into())
            }

            // Emerge: create an agent then immediately wake it
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                let project = context.project()?;
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
                let project = context.project()?;
                let name_str = name.to_string();
                AgentService::remove(project, &name)?;
                Response::new(
                    serde_json::json!({ "type": "receded", "data": name_str }).into(),
                )
            }

            // Status: gather an agent's cognitive context and return it
            Command::Status { name } => {
                let project = context.project()?;
                let ctx = LifecycleService::gather_context(project, &name)?;
                Response::new(
                    serde_json::json!({ "type": "status", "data": ctx }).into(),
                )
            }
        })
    }
}
