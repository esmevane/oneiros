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

    // Event inspection
    #[command(subcommand)]
    Event(EventCommands),
}

impl Command {
    /// Execute a CLI command against the engine context.
    pub fn execute(
        &self,
        context: &EngineContext,
    ) -> Result<Responses, Box<dyn core::error::Error>> {
        match self {
            // Workflow domains — each knows its context
            Command::System(system) => system.execute(&context.system),
            Command::Project(project) => project.execute(
                &context.system,
                context.project.as_ref(),
                &context.brain_name,
            ),
            Command::Seed(seed) => seed.execute(context.project()?),

            // Project-scoped domains
            Command::Level(level) => level.execute(context.project()?),
            Command::Texture(texture) => texture.execute(context.project()?),
            Command::Sensation(sensation) => sensation.execute(context.project()?),
            Command::Nature(nature) => nature.execute(context.project()?),
            Command::Persona(persona) => persona.execute(context.project()?),
            Command::Urge(urge) => urge.execute(context.project()?),
            Command::Agent(agent) => agent.execute(context.project()?),
            Command::Cognition(cognition) => cognition.execute(context.project()?),
            Command::Memory(memory) => memory.execute(context.project()?),
            Command::Experience(experience) => experience.execute(context.project()?),
            Command::Connection(connection) => connection.execute(context.project()?),
            Command::Storage(storage) => storage.execute(context.project()?),
            Command::Search(search) => search.execute(context.project()?),
            Command::Pressure(pressure) => pressure.execute(context.project()?),

            // Doctor — system diagnostics
            Command::Doctor => DoctorCli::execute(&context.system),

            // Lifecycle - available in flat mode as well.
            Command::Lifecycle(lifecycle) => lifecycle.execute(context.project()?),
            Command::Wake { name } => Ok(LifecycleService::wake(context.project()?, &name)?.into()),
            Command::Dream { name } => {
                Ok(LifecycleService::dream(context.project()?, &name)?.into())
            }
            Command::Introspect { name } => {
                Ok(LifecycleService::introspect(context.project()?, &name)?.into())
            }
            Command::Reflect { name } => {
                Ok(LifecycleService::reflect(context.project()?, &name)?.into())
            }
            Command::Sleep { name } => {
                Ok(LifecycleService::sleep(context.project()?, &name)?.into())
            }
            Command::Guidebook { name } => {
                Ok(LifecycleService::guidebook(context.project()?, &name)?.into())
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
                    other => return Err(format!("unexpected agent response: {other:?}").into()),
                };
                LifecycleService::wake(project, &agent_name)?;
                Ok(serde_json::json!({ "type": "emerged", "data": agent_name.to_string() }).into())
            }

            // Recede: retire an agent
            Command::Recede { name } => {
                let project = context.project()?;
                let name_str = name.to_string();
                AgentService::remove(project, &name)?;
                Ok(serde_json::json!({ "type": "receded", "data": name_str }).into())
            }

            // Status: gather an agent's cognitive context and return it
            Command::Status { name } => {
                let project = context.project()?;
                let context = LifecycleService::gather_context(project, &name)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                Ok(serde_json::json!({ "type": "status", "data": context }).into())
            }

            // Event inspection
            Command::Event(cmd) => match cmd {
                EventCommands::List => {
                    let project = context.project()?;
                    let events = project.with_db(event::repo::load_events)?;
                    Ok(serde_json::to_value(&events)?.into())
                }
            },
        }
    }
}
