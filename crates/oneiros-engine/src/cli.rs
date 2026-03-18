//! CLI layer collector — assembles domain commands into a unified command tree.
//!
//! Each domain owns its command definitions and execution in `features/cli.rs`.
//! This module collects them into top-level enums and delegates execution.

use clap::Subcommand;
use serde::Serialize;

use crate::*;

/// Response type for system init workflow.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SystemInitResponse {
    SystemInitialized(TenantName),
    HostAlreadyInitialized,
}

/// Response type for project init workflow.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectInitResponse {
    BrainCreated(BrainName),
    BrainAlreadyExists(BrainName),
}

/// Response type for seed workflow.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SeedResponse {
    SeedComplete,
}

/// Project-level commands (init, etc.).
#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Init {
        #[arg(long, short)]
        yes: bool,
    },
}

/// Seed commands.
#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
}

/// Project-scoped CLI commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize the project — create brain and ticket.
    #[command(subcommand)]
    Project(ProjectCommands),
    /// Seed core vocabulary data.
    #[command(subcommand)]
    Seed(SeedCommands),
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
    #[command(subcommand)]
    Lifecycle(LifecycleCommands),
    #[command(subcommand)]
    Search(SearchCommands),
    #[command(subcommand)]
    Pressure(PressureCommands),
}

/// System-scoped CLI commands.
#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    /// Initialize the system — create tenant and actor.
    Init {
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        yes: bool,
    },
    #[command(subcommand)]
    Tenant(TenantCommands),
    #[command(subcommand)]
    Actor(ActorCommands),
    #[command(subcommand)]
    Brain(BrainCommands),
    #[command(subcommand)]
    Ticket(TicketCommands),
}

/// Execute a project-scoped command.
pub fn execute(
    ctx: &ProjectContext,
    command: Commands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        Commands::Project(_) => {
            // Project init requires a SystemContext, not ProjectContext.
            // This variant should be handled at a higher level.
            Err("project commands require system context — use init_project() directly".into())
        }
        Commands::Seed(SeedCommands::Core) => {
            let result = seed_core(ctx)?;
            Ok(serde_json::to_string_pretty(&result)?)
        }
        Commands::Level(cmd) => LevelCli::execute(ctx, cmd),
        Commands::Texture(cmd) => TextureCli::execute(ctx, cmd),
        Commands::Sensation(cmd) => SensationCli::execute(ctx, cmd),
        Commands::Nature(cmd) => NatureCli::execute(ctx, cmd),
        Commands::Persona(cmd) => PersonaCli::execute(ctx, cmd),
        Commands::Urge(cmd) => UrgeCli::execute(ctx, cmd),
        Commands::Agent(cmd) => AgentCli::execute(ctx, cmd),
        Commands::Cognition(cmd) => CognitionCli::execute(ctx, cmd),
        Commands::Memory(cmd) => MemoryCli::execute(ctx, cmd),
        Commands::Experience(cmd) => ExperienceCli::execute(ctx, cmd),
        Commands::Connection(cmd) => ConnectionCli::execute(ctx, cmd),
        Commands::Lifecycle(cmd) => LifecycleCli::execute(ctx, cmd),
        Commands::Search(cmd) => SearchCli::execute(ctx, cmd),
        Commands::Pressure(cmd) => PressureCli::execute(ctx, cmd),
    }
}

/// Execute a system-scoped command.
pub fn execute_system(
    ctx: &SystemContext,
    command: SystemCommands,
) -> Result<String, Box<dyn std::error::Error>> {
    match command {
        SystemCommands::Init { name, .. } => {
            let result = init_system(ctx, name.unwrap_or_else(|| "onerios user".to_string()))?;
            Ok(serde_json::to_string_pretty(&result)?)
        }
        SystemCommands::Tenant(cmd) => TenantCli::execute(ctx, cmd),
        SystemCommands::Actor(cmd) => ActorCli::execute(ctx, cmd),
        SystemCommands::Brain(cmd) => BrainCli::execute(ctx, cmd),
        SystemCommands::Ticket(cmd) => TicketCli::execute(ctx, cmd),
    }
}

/// System init workflow — create tenant + actor, or report already initialized.
pub fn init_system(
    ctx: &SystemContext,
    name: String,
) -> Result<SystemInitResponse, Box<dyn std::error::Error>> {
    // Check if already initialized
    let tenants = ctx
        .with_db(|conn| TenantRepo::new(conn).list())
        .map_err(|e| format!("database error: {e}"))?;

    if !tenants.is_empty() {
        return Ok(SystemInitResponse::HostAlreadyInitialized);
    }

    let tenant_name = TenantName::new(&name);

    // Create tenant
    TenantService::create(ctx, name.clone())?;

    // Create actor with same name
    let tenants = ctx
        .with_db(|conn| TenantRepo::new(conn).list())
        .map_err(|e| format!("database error: {e}"))?;

    if let Some(tenant) = tenants.first() {
        ActorService::create(ctx, tenant.id.to_string(), name)?;
    }

    Ok(SystemInitResponse::SystemInitialized(tenant_name))
}

/// Project init workflow — create brain + ticket.
pub fn init_project(
    ctx: &SystemContext,
    brain_name: String,
) -> Result<ProjectInitResponse, Box<dyn std::error::Error>> {
    // Check if brain already exists
    if let Ok(BrainResponse::Found(_)) = BrainService::get(ctx, &brain_name) {
        return Ok(ProjectInitResponse::BrainAlreadyExists(BrainName::new(
            &brain_name,
        )));
    }

    BrainService::create(ctx, brain_name.clone())?;

    // Create ticket for the brain
    let actors = ctx
        .with_db(|conn| ActorRepo::new(conn).list())
        .map_err(|e| format!("database error: {e}"))?;

    if let Some(actor) = actors.first() {
        TicketService::create(ctx, actor.id.clone(), brain_name.clone())?;
    }

    Ok(ProjectInitResponse::BrainCreated(BrainName::new(
        &brain_name,
    )))
}

/// Seed core vocabulary data.
pub fn seed_core(
    ctx: &ProjectContext,
) -> Result<SeedResponse, Box<dyn std::error::Error>> {
    // Seed levels
    for (name, description, prompt) in [
        ("working", "What you're actively processing — in-flight thoughts, scratchpad notes, things you haven't consolidated yet.", ""),
        ("session", "Current session context. Learnings, observations, and decisions from what you're doing now.", ""),
        ("project", "Durable knowledge for the lifetime of the project.", ""),
        ("archival", "Deep history — milestone reflections, post-mortems, and historical context.", ""),
        ("core", "Identity fundaments — the memories that define how you process everything else.", ""),
    ] {
        LevelService::set(
            ctx,
            Level {
                name: LevelName::new(name),
                description: description.to_string(),
                prompt: prompt.to_string(),
            },
        )?;
    }

    // Seed textures
    for (name, description) in [
        ("observation", "When you notice something interesting about the code, architecture, or process."),
        ("learning", "Capture moments of genuine understanding."),
        ("question", "Record questions you cannot answer yet."),
        ("connection", "When you see a relationship between separate domains."),
        ("reflection", "Step back and think about how work is going."),
        ("assessment", "Provide a definitive perspective from your domain expertise."),
        ("handoff", "Write what the next session needs to know."),
        ("working", "Capture thoughts as they happen during implementation."),
        ("dream", "Impressions that surface during dreaming."),
        ("bond", "After a meaningful interaction, capture it as a bond."),
    ] {
        TextureService::set(
            ctx,
            Texture {
                name: TextureName::new(name),
                description: description.to_string(),
                prompt: String::new(),
            },
        )?;
    }

    // Seed sensations
    for (name, description) in [
        ("caused", "Directed. One thought produced another."),
        ("continues", "Directed. Picks up where a previous thread left off."),
        ("distills", "Directed. A consolidated understanding formed from earlier raw thoughts."),
        ("echoes", "Undirected. Things that resonate thematically without clear causation."),
        ("grounds", "Directed. A thought grounded in a memory or prior knowledge."),
        ("tensions", "Undirected. Ideas that pull against each other."),
    ] {
        SensationService::set(
            ctx,
            Sensation {
                name: SensationName::new(name),
                description: description.to_string(),
                prompt: String::new(),
            },
        )?;
    }

    // Seed natures
    for (name, description) in [
        ("context", "Directed. One entity provides background or framing for another."),
        ("continuation", "Directed. One entity follows from another."),
        ("contrast", "Undirected. Two entities differ in an instructive way."),
        ("origin", "Directed. One entity gave rise to another."),
        ("reference", "Undirected. Two entities are related without implying causation."),
        ("revision", "Directed. One entity updates or supersedes another."),
    ] {
        NatureService::set(
            ctx,
            Nature {
                name: NatureName::new(name),
                description: description.to_string(),
                prompt: String::new(),
            },
        )?;
    }

    // Seed personas
    for (name, description) in [
        ("process", "Internal lifecycle agents — orchestration, session structure, cognitive hygiene."),
        ("expert", "Domain specialists — deep knowledge in a specific area."),
        ("scribe", "Record-keepers — maintain the cognitive record."),
    ] {
        PersonaService::set(
            ctx,
            Persona {
                name: PersonaName::new(name),
                description: description.to_string(),
                prompt: String::new(),
            },
        )?;
    }

    // Seed urges
    for (name, description, prompt) in [
        ("introspect", "The pull to look inward.", "Pause to examine your cognitive state."),
        ("catharsis", "The need to release accumulated tension.", "Confront what's stuck and clear space."),
        ("recollect", "The drive to gather scattered pieces.", "Search for connections and consolidate."),
        ("retrospect", "The impulse to look back.", "Review the trajectory and capture learnings."),
    ] {
        UrgeService::set(
            ctx,
            Urge {
                name: UrgeName::new(name),
                description: description.to_string(),
                prompt: prompt.to_string(),
            },
        )?;
    }

    Ok(SeedResponse::SeedComplete)
}
