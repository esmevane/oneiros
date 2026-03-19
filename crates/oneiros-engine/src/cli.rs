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
        name: String,
    },
    Dream {
        name: String,
    },
    Introspect {
        name: String,
    },
    Reflect {
        name: String,
    },
    Sleep {
        name: String,
    },
    Guidebook {
        name: String,
    },

    // Agent lifecycle — emerge creates then wakes; recede removes
    Emerge {
        name: String,
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
    },
    Recede {
        name: String,
    },

    // Status — summary of an agent's current cognitive state
    Status {
        name: String,
    },

    // Diagnostics
    Doctor,

    // Event inspection
    #[command(subcommand)]
    Event(EventCommands),
}

/// Event inspection commands.
#[derive(Debug, Subcommand)]
pub enum EventCommands {
    /// List all events in the project event log.
    List,
}

/// The combined context for CLI execution.
///
/// Holds both system and project contexts. System context is always
/// available. Project context is available after `project init` and
/// `start_service`.
pub struct EngineContext {
    pub system: SystemContext,
    pub project: Option<ProjectContext>,
    pub brain_name: String,
}

impl EngineContext {
    pub fn project(&self) -> Result<&ProjectContext, Box<dyn std::error::Error>> {
        self.project
            .as_ref()
            .ok_or_else(|| "project context required — call start_service first".into())
    }
}

/// Execute a CLI command against the engine context.
pub fn execute(
    ctx: &EngineContext,
    command: Command,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        // Workflow domains — each knows its context
        Command::System(cmd) => SystemCli::execute(&ctx.system, cmd),
        Command::Project(cmd) => ProjectCli::execute(&ctx.system, &ctx.brain_name, cmd),
        Command::Seed(cmd) => SeedCli::execute(ctx.project()?, cmd),

        // Project-scoped domains
        Command::Level(cmd) => LevelCli::execute(ctx.project()?, cmd),
        Command::Texture(cmd) => TextureCli::execute(ctx.project()?, cmd),
        Command::Sensation(cmd) => SensationCli::execute(ctx.project()?, cmd),
        Command::Nature(cmd) => NatureCli::execute(ctx.project()?, cmd),
        Command::Persona(cmd) => PersonaCli::execute(ctx.project()?, cmd),
        Command::Urge(cmd) => UrgeCli::execute(ctx.project()?, cmd),
        Command::Agent(cmd) => AgentCli::execute(ctx.project()?, cmd),
        Command::Cognition(cmd) => CognitionCli::execute(ctx.project()?, cmd),
        Command::Memory(cmd) => MemoryCli::execute(ctx.project()?, cmd),
        Command::Experience(cmd) => ExperienceCli::execute(ctx.project()?, cmd),
        Command::Connection(cmd) => ConnectionCli::execute(ctx.project()?, cmd),
        Command::Lifecycle(cmd) => LifecycleCli::execute(ctx.project()?, cmd),
        Command::Storage(cmd) => StorageCli::execute(ctx.project()?, cmd),
        Command::Search(cmd) => SearchCli::execute(ctx.project()?, cmd),
        Command::Pressure(cmd) => PressureCli::execute(ctx.project()?, cmd),

        // Doctor — system diagnostics
        Command::Doctor => DoctorCli::execute(&ctx.system),

        // Flat lifecycle shortcuts
        Command::Wake { name } => {
            let response = LifecycleService::wake(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }
        Command::Dream { name } => {
            let response = LifecycleService::dream(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }
        Command::Introspect { name } => {
            let response = LifecycleService::introspect(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }
        Command::Reflect { name } => {
            let response = LifecycleService::reflect(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }
        Command::Sleep { name } => {
            let response = LifecycleService::sleep(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }
        Command::Guidebook { name } => {
            let response = LifecycleService::guidebook(ctx.project()?, &name)?;
            Ok(serde_json::to_string_pretty(&response)?)
        }

        // Emerge: create an agent then immediately wake it
        Command::Emerge {
            name,
            persona,
            description,
        } => {
            let project = ctx.project()?;
            let created = AgentService::create(project, name, persona, description, String::new())?;
            let agent_name = match &created {
                AgentResponse::AgentCreated(n) => n.to_string(),
                other => return Err(format!("unexpected agent response: {other:?}").into()),
            };
            LifecycleService::wake(project, &agent_name)?;
            let payload = serde_json::json!({ "type": "emerged", "data": agent_name });
            Ok(serde_json::to_string_pretty(&payload)?)
        }

        // Recede: retire an agent
        Command::Recede { name } => {
            let project = ctx.project()?;
            AgentService::remove(project, &name)?;
            let payload = serde_json::json!({ "type": "receded", "data": name });
            Ok(serde_json::to_string_pretty(&payload)?)
        }

        // Status: gather an agent's cognitive context and return it
        Command::Status { name } => {
            let project = ctx.project()?;
            let context = LifecycleService::gather_context(project, &name)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let payload = serde_json::json!({ "type": "status", "data": context });
            Ok(serde_json::to_string_pretty(&payload)?)
        }

        // Event inspection
        Command::Event(cmd) => match cmd {
            EventCommands::List => {
                let project = ctx.project()?;
                let events = project.with_db(load_events)?;
                Ok(serde_json::to_string_pretty(&events)?)
            }
        },
    }
}
