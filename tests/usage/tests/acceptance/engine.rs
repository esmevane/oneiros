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

impl Cli {
    fn execute(&self, context: &EngineContext) -> Result<Responses, Box<dyn core::error::Error>> {
        Ok(self.command.execute(context)?)
    }
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

        let response = Cli::try_parse_from(&full_args)?.execute(&self.ctx)?;
        let value = serde_json::to_value(&response)?;

        // If the result is already an array, use it directly.
        // Otherwise wrap in an array for consistency.
        if value.is_array() {
            Ok(value)
        } else {
            Ok(serde_json::Value::Array(vec![value]))
        }
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let brain_db_path = self._temp.path().join("brain.db");

        let conn = rusqlite::Connection::open(&brain_db_path)?;
        migrate_project(&conn)?;

        let data_dir = self._temp.path().join("data");
        std::fs::create_dir_all(&data_dir)?;

        self.ctx.project = Some(
            ProjectContext::new(conn, PROJECT_PROJECTIONS).with_config(Config::new(&data_dir)),
        );

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

// Agent
#[tokio::test]
async fn agent_create_with_persona() -> TestResult {
    cases::agent::create_with_persona::<Engine>().await
}
#[tokio::test]
async fn agent_show_returns_details() -> TestResult {
    cases::agent::show_returns_details::<Engine>().await
}
#[tokio::test]
async fn agent_list_empty() -> TestResult {
    cases::agent::list_empty::<Engine>().await
}
#[tokio::test]
async fn agent_list_populated() -> TestResult {
    cases::agent::list_populated::<Engine>().await
}
#[tokio::test]
async fn agent_update_changes_fields() -> TestResult {
    cases::agent::update_changes_fields::<Engine>().await
}
#[tokio::test]
async fn agent_remove_makes_it_unlisted() -> TestResult {
    cases::agent::remove_makes_it_unlisted::<Engine>().await
}
#[tokio::test]
async fn agent_name_includes_persona_suffix() -> TestResult {
    cases::agent::name_includes_persona_suffix::<Engine>().await
}

// Connection
#[tokio::test]
async fn connection_create() -> TestResult {
    cases::connection::create::<Engine>().await
}
#[tokio::test]
async fn connection_list_empty() -> TestResult {
    cases::connection::list_empty::<Engine>().await
}
#[tokio::test]
async fn connection_list_populated() -> TestResult {
    cases::connection::list_populated::<Engine>().await
}
#[tokio::test]
async fn connection_show_by_id() -> TestResult {
    cases::connection::show_by_id::<Engine>().await
}
#[tokio::test]
async fn connection_remove_by_id() -> TestResult {
    cases::connection::remove_by_id::<Engine>().await
}

// Cognition
#[tokio::test]
async fn cognition_add() -> TestResult {
    cases::cognition::add_creates_cognition::<Engine>().await
}
#[tokio::test]
async fn cognition_list_empty() -> TestResult {
    cases::cognition::list_empty::<Engine>().await
}
#[tokio::test]
async fn cognition_list_populated() -> TestResult {
    cases::cognition::list_populated::<Engine>().await
}
#[tokio::test]
async fn cognition_list_filters_by_agent() -> TestResult {
    cases::cognition::list_filters_by_agent::<Engine>().await
}
#[tokio::test]
async fn cognition_show_by_id() -> TestResult {
    cases::cognition::show_by_id::<Engine>().await
}

// Memory
#[tokio::test]
async fn memory_add() -> TestResult {
    cases::memory::add_creates_memory::<Engine>().await
}
#[tokio::test]
async fn memory_list_empty() -> TestResult {
    cases::memory::list_empty::<Engine>().await
}
#[tokio::test]
async fn memory_list_populated() -> TestResult {
    cases::memory::list_populated::<Engine>().await
}
#[tokio::test]
async fn memory_list_filters_by_agent() -> TestResult {
    cases::memory::list_filters_by_agent::<Engine>().await
}
#[tokio::test]
async fn memory_show_by_id() -> TestResult {
    cases::memory::show_by_id::<Engine>().await
}

// Experience
#[tokio::test]
async fn experience_create() -> TestResult {
    cases::experience::create::<Engine>().await
}
#[tokio::test]
async fn experience_list_empty() -> TestResult {
    cases::experience::list_empty::<Engine>().await
}
#[tokio::test]
async fn experience_list_populated() -> TestResult {
    cases::experience::list_populated::<Engine>().await
}
#[tokio::test]
async fn experience_show_by_id() -> TestResult {
    cases::experience::show_by_id::<Engine>().await
}
#[tokio::test]
async fn experience_update_description() -> TestResult {
    cases::experience::update_description::<Engine>().await
}

// Lifecycle
#[tokio::test]
async fn lifecycle_wake() -> TestResult {
    cases::lifecycle::wake::<Engine>().await
}
#[tokio::test]
async fn lifecycle_dream() -> TestResult {
    cases::lifecycle::dream::<Engine>().await
}
#[tokio::test]
async fn lifecycle_introspect() -> TestResult {
    cases::lifecycle::introspect::<Engine>().await
}
#[tokio::test]
async fn lifecycle_reflect() -> TestResult {
    cases::lifecycle::reflect::<Engine>().await
}
#[tokio::test]
async fn lifecycle_sleep() -> TestResult {
    cases::lifecycle::sleep::<Engine>().await
}
#[tokio::test]
async fn lifecycle_guidebook() -> TestResult {
    cases::lifecycle::guidebook::<Engine>().await
}

// Emerge / Recede
#[tokio::test]
async fn emerge_creates_and_wakes_agent() -> TestResult {
    cases::emerge::creates_and_wakes_agent::<Engine>().await
}
#[tokio::test]
async fn recede_retires_agent() -> TestResult {
    cases::emerge::recede_retires_agent::<Engine>().await
}

// Status
#[tokio::test]
async fn status_returns_agent_status() -> TestResult {
    cases::status::returns_agent_status::<Engine>().await
}

// Event
#[tokio::test]
async fn event_list_shows_events() -> TestResult {
    cases::event::list_shows_events::<Engine>().await
}

// Doctor
#[tokio::test]
async fn doctor_reports_initialized() -> TestResult {
    cases::doctor::reports_initialized_system::<Engine>().await
}
#[tokio::test]
async fn doctor_reports_uninitialized() -> TestResult {
    cases::doctor::reports_uninitialized_system::<Engine>().await
}

// Search
#[tokio::test]
async fn search_finds_cognition_content() -> TestResult {
    cases::search::finds_cognition_content::<Engine>().await
}
#[tokio::test]
async fn search_finds_memory_content() -> TestResult {
    cases::search::finds_memory_content::<Engine>().await
}
#[tokio::test]
async fn search_finds_experience_description() -> TestResult {
    cases::search::finds_experience_description::<Engine>().await
}
#[tokio::test]
async fn search_finds_agent_description() -> TestResult {
    cases::search::finds_agent_description::<Engine>().await
}
#[tokio::test]
async fn search_finds_persona_description() -> TestResult {
    cases::search::finds_persona_description::<Engine>().await
}
#[tokio::test]
async fn search_returns_empty_for_no_match() -> TestResult {
    cases::search::returns_empty_for_no_match::<Engine>().await
}
#[tokio::test]
async fn search_filters_by_agent() -> TestResult {
    cases::search::filters_by_agent::<Engine>().await
}

// Pressure
#[tokio::test]
async fn pressure_returns_readings() -> TestResult {
    cases::pressure::returns_readings_for_agent::<Engine>().await
}
#[tokio::test]
async fn pressure_introspect_decreases_after_introspecting() -> TestResult {
    cases::pressure::introspect_pressure_decreases_after_introspecting::<Engine>().await
}
#[tokio::test]
async fn pressure_catharsis_decreases_after_reflecting() -> TestResult {
    cases::pressure::catharsis_pressure_decreases_after_reflecting::<Engine>().await
}

// Storage
#[tokio::test]
async fn storage_set_and_show() -> TestResult {
    cases::storage::set_and_show::<Engine>().await
}
#[tokio::test]
async fn storage_list_empty() -> TestResult {
    cases::storage::list_empty::<Engine>().await
}
#[tokio::test]
async fn storage_list_populated() -> TestResult {
    cases::storage::list_populated::<Engine>().await
}
#[tokio::test]
async fn storage_remove() -> TestResult {
    cases::storage::remove::<Engine>().await
}

// Texture
#[tokio::test]
async fn texture_set_creates() -> TestResult {
    cases::texture::set_creates::<Engine>().await
}
#[tokio::test]
async fn texture_set_updates() -> TestResult {
    cases::texture::set_updates::<Engine>().await
}
#[tokio::test]
async fn texture_list_empty() -> TestResult {
    cases::texture::list_empty::<Engine>().await
}
#[tokio::test]
async fn texture_list_populated() -> TestResult {
    cases::texture::list_populated::<Engine>().await
}
#[tokio::test]
async fn texture_remove() -> TestResult {
    cases::texture::remove::<Engine>().await
}

// Sensation
#[tokio::test]
async fn sensation_set_creates() -> TestResult {
    cases::sensation::set_creates::<Engine>().await
}
#[tokio::test]
async fn sensation_set_updates() -> TestResult {
    cases::sensation::set_updates::<Engine>().await
}
#[tokio::test]
async fn sensation_list_empty() -> TestResult {
    cases::sensation::list_empty::<Engine>().await
}
#[tokio::test]
async fn sensation_list_populated() -> TestResult {
    cases::sensation::list_populated::<Engine>().await
}
#[tokio::test]
async fn sensation_remove() -> TestResult {
    cases::sensation::remove::<Engine>().await
}

// Nature
#[tokio::test]
async fn nature_set_creates() -> TestResult {
    cases::nature::set_creates::<Engine>().await
}
#[tokio::test]
async fn nature_set_updates() -> TestResult {
    cases::nature::set_updates::<Engine>().await
}
#[tokio::test]
async fn nature_list_empty() -> TestResult {
    cases::nature::list_empty::<Engine>().await
}
#[tokio::test]
async fn nature_list_populated() -> TestResult {
    cases::nature::list_populated::<Engine>().await
}
#[tokio::test]
async fn nature_remove() -> TestResult {
    cases::nature::remove::<Engine>().await
}

// Persona
#[tokio::test]
async fn persona_set_creates() -> TestResult {
    cases::persona::set_creates::<Engine>().await
}
#[tokio::test]
async fn persona_set_updates() -> TestResult {
    cases::persona::set_updates::<Engine>().await
}
#[tokio::test]
async fn persona_list_empty() -> TestResult {
    cases::persona::list_empty::<Engine>().await
}
#[tokio::test]
async fn persona_list_populated() -> TestResult {
    cases::persona::list_populated::<Engine>().await
}
#[tokio::test]
async fn persona_remove() -> TestResult {
    cases::persona::remove::<Engine>().await
}

// Urge
#[tokio::test]
async fn urge_set_creates() -> TestResult {
    cases::urge::set_creates::<Engine>().await
}
#[tokio::test]
async fn urge_set_updates() -> TestResult {
    cases::urge::set_updates::<Engine>().await
}
#[tokio::test]
async fn urge_list_empty() -> TestResult {
    cases::urge::list_empty::<Engine>().await
}
#[tokio::test]
async fn urge_list_populated() -> TestResult {
    cases::urge::list_populated::<Engine>().await
}
#[tokio::test]
async fn urge_remove() -> TestResult {
    cases::urge::remove::<Engine>().await
}

// Import / Export
#[tokio::test]
async fn export_produces_file() -> TestResult {
    cases::import_export::export_produces_file::<Engine>().await
}
#[tokio::test]
async fn import_restores_data() -> TestResult {
    cases::import_export::import_restores_data::<Engine>().await
}
#[tokio::test]
async fn replay_rebuilds_projections() -> TestResult {
    cases::import_export::replay_rebuilds_projections::<Engine>().await
}
