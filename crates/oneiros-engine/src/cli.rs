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
    pub async fn execute(&self, engine: &Engine) -> Result<Response<Responses>, Error> {
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
            Command::Level(level) => Response::new(level.execute(engine.project()?).await?),
            Command::Texture(texture) => Response::new(texture.execute(engine.project()?).await?),
            Command::Sensation(sensation) => {
                Response::new(sensation.execute(engine.project()?).await?)
            }
            Command::Nature(nature) => Response::new(nature.execute(engine.project()?).await?),
            Command::Persona(persona) => Response::new(persona.execute(engine.project()?).await?),
            Command::Urge(urge) => Response::new(urge.execute(engine.project()?).await?),
            Command::Agent(agent) => Response::new(agent.execute(engine.project()?).await?),

            // Entity domains — return Response<Responses> with ref_token in meta
            Command::Cognition(cognition) => cognition.execute(engine.project()?).await?,
            Command::Memory(memory) => memory.execute(engine.project()?).await?,
            Command::Experience(experience) => experience.execute(engine.project()?).await?,
            Command::Connection(connection) => connection.execute(engine.project()?).await?,

            Command::Storage(storage) => Response::new(storage.execute(engine.project()?).await?),
            Command::Search(search) => Response::new(search.execute(engine.project()?).await?),
            Command::Pressure(pressure) => {
                Response::new(pressure.execute(engine.project()?).await?)
            }

            // Doctor — system diagnostics
            Command::Doctor => Response::new(
                DoctorCli::execute(engine.system()).map_err(|e| Error::Context(e.to_string()))?,
            ),

            // Lifecycle — domain subcommands go through the client
            Command::Lifecycle(lifecycle) => {
                Response::new(lifecycle.execute(engine.project()?).await?)
            }

            // Flat lifecycle shortcuts — go through the lifecycle client
            Command::Wake { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .wake(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Dream { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .dream(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Introspect { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .introspect(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Reflect { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .reflect(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Sleep { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .sleep(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Guidebook { name } => {
                let project = engine.project()?;
                let client = project.client();
                let lifecycle_client = LifecycleClient::new(&client);
                Response::new(
                    lifecycle_client
                        .guidebook(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }

            // Emerge: create an agent then immediately wake it.
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                let project = engine.project()?;
                let client = project.client();
                let agent_client = AgentClient::new(&client);
                let lifecycle_client = LifecycleClient::new(&client);

                let created = agent_client
                    .create(
                        name.clone(),
                        persona.clone(),
                        description.clone(),
                        Prompt::new(""),
                    )
                    .await
                    .map_err(|e| Error::Context(e.to_string()))?;

                let agent_name = match &created {
                    AgentResponse::AgentCreated(n) => n.clone(),
                    other => {
                        return Err(Error::Context(format!(
                            "unexpected agent response: {other:?}"
                        )));
                    }
                };

                lifecycle_client
                    .wake(&agent_name)
                    .await
                    .map_err(|e| Error::Context(e.to_string()))?;

                Response::new(
                    serde_json::json!({ "type": "emerged", "data": agent_name.to_string() }).into(),
                )
            }

            // Recede: retire an agent via the client.
            Command::Recede { name } => {
                let project = engine.project()?;
                let client = project.client();
                let agent_client = AgentClient::new(&client);
                let name_str = name.to_string();
                agent_client
                    .remove(name)
                    .await
                    .map_err(|e| Error::Context(e.to_string()))?;
                Response::new(serde_json::json!({ "type": "receded", "data": name_str }).into())
            }

            // Status: gather an agent's cognitive context locally.
            //
            // NOTE: gather_context has no HTTP route — this is a read-only
            // composition of multiple domain reads and is fulfilled directly
            // from the local database. If a /status/{agent} or
            // /context/{agent} route is added, this should move to the client.
            Command::Status { name } => {
                let project = engine.project()?;
                let context = LifecycleService::gather_context(project, name)
                    .map_err(|e| Error::Context(e.to_string()))?;
                Response::new(serde_json::json!({ "type": "status", "data": context }).into())
            }
        })
    }
}
