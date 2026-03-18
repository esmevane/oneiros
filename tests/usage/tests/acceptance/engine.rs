use clap::Parser;
use oneiros_engine::*;
use oneiros_usage::*;

use crate::cases;

pub struct Engine {
    _temp: tempfile::TempDir,
    system_ctx: SystemContext,
    project_ctx: Option<ProjectContext>,
    server: Option<tokio::task::JoinHandle<()>>,
}

/// Unified CLI wrapper for parsing engine commands from strings.
///
/// The engine doesn't have a unified CLI entry point — it has separate
/// `Commands` (project) and `SystemCommands` (system) enums. This wrapper
/// adds the top-level routing the test harness needs.
#[derive(Debug, Parser)]
#[command(name = "oneiros")]
struct EngineCli {
    #[command(subcommand)]
    command: EngineCommand,
}

#[derive(Debug, clap::Subcommand)]
enum EngineCommand {
    /// System-scoped commands.
    #[command(subcommand)]
    System(EngineSystemCommands),

    /// Project-level commands.
    #[command(subcommand)]
    Project(EngineProjectCommands),

    /// Seed commands.
    #[command(subcommand)]
    Seed(EngineSeedCommands),

    /// Project-scoped commands — delegates to engine Commands.
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

#[derive(Debug, clap::Subcommand)]
enum EngineProjectCommands {
    Init {
        #[arg(long, short)]
        yes: bool,
    },
}

#[derive(Debug, clap::Subcommand)]
enum EngineSeedCommands {
    Core,
}

#[derive(Debug, clap::Subcommand)]
enum EngineSystemCommands {
    /// Initialize the system (create tenant + actor).
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

/// Project-level projections for the engine.
static PROJECT_PROJECTIONS: &[&[Projection]] = &[
    LevelProjections.all(),
    TextureProjections.all(),
    SensationProjections.all(),
    NatureProjections.all(),
    PersonaProjections.all(),
    UrgeProjections.all(),
    AgentProjections.all(),
    CognitionProjections.all(),
    MemoryProjections.all(),
    ExperienceProjections.all(),
    ConnectionProjections.all(),
    PressureProjections.all(),
    SearchProjections.all(),
    StorageProjections.all(),
];

static SYSTEM_PROJECTIONS: &[&[Projection]] = &[
    TenantProjections.all(),
    ActorProjections.all(),
    BrainProjections.all(),
    TicketProjections.all(),
];

impl Backend for Engine {
    async fn start() -> Result<Self, Box<dyn core::error::Error>> {
        let temp = tempfile::TempDir::new()?;
        let db_path = temp.path().join("system.db");

        let conn = rusqlite::Connection::open(&db_path)?;
        migrate_system(&conn)?;

        let system_ctx = SystemContext::new(conn, SYSTEM_PROJECTIONS);

        Ok(Self {
            _temp: temp,
            system_ctx,
            project_ctx: None,
            server: None,
        })
    }

    async fn exec(
        &self,
        command: &str,
    ) -> Result<serde_json::Value, Box<dyn core::error::Error>> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        // Strip --output json / -o json since the engine always returns JSON
        let full_args = strip_output_flag(full_args);

        let cli = EngineCli::try_parse_from(&full_args)?;

        let json_string = match cli.command {
            EngineCommand::System(sys_cmd) => {
                let engine_cmd = match sys_cmd {
                    EngineSystemCommands::Init { name, yes } => {
                        SystemCommands::Init { name, yes }
                    }
                    EngineSystemCommands::Tenant(cmd) => SystemCommands::Tenant(cmd),
                    EngineSystemCommands::Actor(cmd) => SystemCommands::Actor(cmd),
                    EngineSystemCommands::Brain(cmd) => SystemCommands::Brain(cmd),
                    EngineSystemCommands::Ticket(cmd) => SystemCommands::Ticket(cmd),
                };
                execute_system(&self.system_ctx, engine_cmd)?
            }
            EngineCommand::Project(proj_cmd) => match proj_cmd {
                EngineProjectCommands::Init { .. } => {
                    let result =
                        init_project(&self.system_ctx, "test-project".to_string())?;
                    serde_json::to_string_pretty(&result)?
                }
            },
            EngineCommand::Seed(seed_cmd) => match seed_cmd {
                EngineSeedCommands::Core => {
                    let project_ctx = self
                        .project_ctx
                        .as_ref()
                        .expect("project context required — call start_service first");
                    let result = seed_core(project_ctx)?;
                    serde_json::to_string_pretty(&result)?
                }
            },
            cmd => {
                let project_ctx = self.project_ctx.as_ref()
                    .expect("project context required — call start_service first");
                let engine_cmd = match cmd {
                    EngineCommand::Level(c) => Commands::Level(c),
                    EngineCommand::Texture(c) => Commands::Texture(c),
                    EngineCommand::Sensation(c) => Commands::Sensation(c),
                    EngineCommand::Nature(c) => Commands::Nature(c),
                    EngineCommand::Persona(c) => Commands::Persona(c),
                    EngineCommand::Urge(c) => Commands::Urge(c),
                    EngineCommand::Agent(c) => Commands::Agent(c),
                    EngineCommand::Cognition(c) => Commands::Cognition(c),
                    EngineCommand::Memory(c) => Commands::Memory(c),
                    EngineCommand::Experience(c) => Commands::Experience(c),
                    EngineCommand::Connection(c) => Commands::Connection(c),
                    EngineCommand::Lifecycle(c) => Commands::Lifecycle(c),
                    EngineCommand::Search(c) => Commands::Search(c),
                    EngineCommand::Pressure(c) => Commands::Pressure(c),
                    EngineCommand::System(_)
                    | EngineCommand::Project(_)
                    | EngineCommand::Seed(_) => unreachable!(),
                };
                execute(project_ctx, engine_cmd)?
            }
        };

        let value: serde_json::Value = serde_json::from_str(&json_string)?;

        // Wrap in array to match legacy output format
        Ok(serde_json::Value::Array(vec![value]))
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let brain_db_path = self._temp.path().join("brain.db");

        let conn = rusqlite::Connection::open(&brain_db_path)?;
        migrate_project(&conn)?;

        self.project_ctx = Some(ProjectContext::new(conn, PROJECT_PROJECTIONS));

        // The engine has project_router() but for now the CLI dispatches directly
        // through execute(). When the test cases require HTTP client→server path,
        // we'll start the server here.

        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Some(handle) = &self.server {
            handle.abort();
        }
    }
}

/// Strip `--output json` or `-o json` flag pairs from args.
fn strip_output_flag(args: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut skip_next = false;

    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--output" || arg == "-o" {
            skip_next = true;
            continue;
        }
        result.push(arg);
    }

    result
}

/// Split a command string into words, respecting single and double quotes.
fn shell_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_quote: Option<char> = None;

    for ch in input.chars() {
        match (ch, in_quote) {
            ('\'' | '"', None) => in_quote = Some(ch),
            (q, Some(open)) if q == open => in_quote = None,
            (' ' | '\t', None) => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

#[tokio::test]
async fn system_init_creates_tenant_and_actor() -> TestResult {
    cases::system::init_creates_tenant_and_actor::<Engine>().await
}

#[tokio::test]
async fn system_init_is_idempotent() -> TestResult {
    cases::system::init_is_idempotent::<Engine>().await
}

#[tokio::test]
async fn project_init_creates_brain() -> TestResult {
    cases::project::init_creates_brain::<Engine>().await
}

#[tokio::test]
async fn level_set_creates_a_new_level() -> TestResult {
    cases::level::set_creates_a_new_level::<Engine>().await
}

#[tokio::test]
async fn level_set_updates_existing_level() -> TestResult {
    cases::level::set_updates_existing_level::<Engine>().await
}

#[tokio::test]
async fn level_list_returns_empty_when_none_exist() -> TestResult {
    cases::level::list_returns_empty_when_none_exist::<Engine>().await
}

#[tokio::test]
async fn level_list_returns_created_levels() -> TestResult {
    cases::level::list_returns_created_levels::<Engine>().await
}

#[tokio::test]
async fn level_remove_makes_it_unlisted() -> TestResult {
    cases::level::remove_makes_it_unlisted::<Engine>().await
}

#[tokio::test]
async fn seed_core_creates_default_levels() -> TestResult {
    cases::seed::core_creates_default_levels::<Engine>().await
}
