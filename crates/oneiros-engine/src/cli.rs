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

            // Continuity — domain subcommands go through the client
            Command::Continuity(continuity) => {
                Response::new(continuity.execute(engine.project()?).await?)
            }

            // Flat lifecycle shortcuts — go through the lifecycle client
            Command::Wake { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .wake(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Dream { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .dream(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Introspect { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .introspect(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Reflect { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .reflect(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Sleep { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .sleep(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
            Command::Guidebook { name } => {
                let project = engine.project()?;
                let client = project.client();
                let continuity_client = ContinuityClient::new(&client);
                Response::new(
                    continuity_client
                        .guidebook(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }

            // Continuity lifecycle — emerge, recede, status
            Command::Emerge {
                name,
                persona,
                description,
            } => {
                let client = engine.project()?.client();
                let continuity = ContinuityClient::new(&client);
                Response::new(
                    continuity
                        .emerge(name.clone(), persona.clone(), description.clone())
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }

            Command::Recede { name } => {
                let client = engine.project()?.client();
                let continuity = ContinuityClient::new(&client);
                Response::new(
                    continuity
                        .recede(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }

            Command::Status { name } => {
                let client = engine.project()?.client();
                let continuity = ContinuityClient::new(&client);
                Response::new(
                    continuity
                        .status(name)
                        .await
                        .map_err(|e| Error::Context(e.to_string()))?
                        .into(),
                )
            }
        })
    }
}
