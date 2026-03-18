use clap::Parser;
use oneiros_engine::*;
use oneiros_usage::*;

use crate::cases;

pub struct Engine {
    _temp: tempfile::TempDir,
    ctx: EngineContext,
    server: Option<tokio::task::JoinHandle<()>>,
}

/// Wrapper to parse command strings into the engine's `Command` enum.
#[derive(Debug, Parser)]
#[command(name = "oneiros")]
struct Cli {
    #[command(subcommand)]
    command: Command,
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

        let ctx = EngineContext {
            system: system_ctx,
            project: None,
            brain_name: "test-project".to_string(),
        };

        Ok(Self {
            _temp: temp,
            ctx,
            server: None,
        })
    }

    async fn exec(&self, command: &str) -> Result<serde_json::Value, Box<dyn core::error::Error>> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        let full_args = strip_output_flag(full_args);

        let cli = Cli::try_parse_from(&full_args)?;
        let json_string = execute(&self.ctx, cli.command)?;
        let value: serde_json::Value = serde_json::from_str(&json_string)?;

        Ok(serde_json::Value::Array(vec![value]))
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let brain_db_path = self._temp.path().join("brain.db");

        let conn = rusqlite::Connection::open(&brain_db_path)?;
        migrate_project(&conn)?;

        self.ctx.project = Some(ProjectContext::new(conn, PROJECT_PROJECTIONS));

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
