use clap::Parser;
use oneiros_engine::*;
use oneiros_usage::*;

use crate::*;

pub struct EngineBackend {
    config: Config,
    _dir: tempfile::TempDir,
    server: Option<tokio::task::JoinHandle<()>>,
}

impl Backend for EngineBackend {
    async fn start() -> Result<Self, Box<dyn core::error::Error>> {
        let dir = tempfile::TempDir::new()?;
        let mut config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .brain(BrainName::new("test-project"))
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse()?)
                    .build(),
            )
            .build();

        config.bootstrap()?;

        // Start the HTTP server eagerly — the engine CLI routes all
        // commands through HTTP clients, so the service must be running
        // before any commands execute.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        config.service.address = addr;

        let server_config = config.clone();
        let server = tokio::spawn(async move {
            Server::new(server_config)
                .serve(listener)
                .await
                .expect("server failed");
        });

        Ok(Self {
            config,
            _dir: dir,
            server: Some(server),
        })
    }

    async fn exec_json(&self, command: &str) -> Result<Response<Responses>, Error> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        let cli = Cli::try_parse_from(&full_args).map_err(|e| Error::Context(e.to_string()))?;
        let rendered = cli.execute(&self.config).await?;

        Ok(rendered.into_response())
    }

    async fn exec_prompt(&self, command: &str) -> Result<String, Error> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        let cli = Cli::try_parse_from(&full_args).map_err(|e| Error::Context(e.to_string()))?;
        let rendered = cli.execute(&self.config).await?;

        Ok(rendered.prompt().to_string())
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        // Server is started eagerly in start() — this is a no-op.
        Ok(())
    }
}

impl Drop for EngineBackend {
    fn drop(&mut self) {
        if let Some(handle) = self.server.take() {
            handle.abort();
        }
    }
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
    cases::system::init_creates_tenant_and_actor::<EngineBackend>().await
}

#[tokio::test]
async fn system_init_is_idempotent() -> TestResult {
    cases::system::init_is_idempotent::<EngineBackend>().await
}
#[tokio::test]
async fn system_init_prompt() -> TestResult {
    cases::system::init_prompt::<EngineBackend>().await
}

// Tenant
#[tokio::test]
async fn tenant_list_after_system_init() -> TestResult {
    cases::tenant::list_after_system_init::<EngineBackend>().await
}
#[tokio::test]
async fn tenant_list_prompt() -> TestResult {
    cases::tenant::list_prompt::<EngineBackend>().await
}

// Actor (system-scoped)
#[tokio::test]
async fn actor_list_after_system_init() -> TestResult {
    cases::actor::list_after_system_init::<EngineBackend>().await
}
#[tokio::test]
async fn actor_list_prompt() -> TestResult {
    cases::actor::list_prompt::<EngineBackend>().await
}

#[tokio::test]
async fn project_init_prompt() -> TestResult {
    cases::project::init_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn project_init_creates_brain() -> TestResult {
    cases::project::init_creates_brain::<EngineBackend>().await
}

// Brain
#[tokio::test]
async fn brain_list_after_project_init() -> TestResult {
    cases::brain::list_after_project_init::<EngineBackend>().await
}
#[tokio::test]
async fn brain_get_by_name() -> TestResult {
    cases::brain::get_by_name::<EngineBackend>().await
}
#[tokio::test]
async fn brain_list_prompt() -> TestResult {
    cases::brain::list_prompt::<EngineBackend>().await
}

// Ticket
#[tokio::test]
async fn ticket_list_after_project_init() -> TestResult {
    cases::ticket::list_after_project_init::<EngineBackend>().await
}
#[tokio::test]
async fn ticket_list_prompt() -> TestResult {
    cases::ticket::list_prompt::<EngineBackend>().await
}

#[tokio::test]
async fn level_set_creates_a_new_level() -> TestResult {
    cases::level::set_creates_a_new_level::<EngineBackend>().await
}

#[tokio::test]
async fn level_set_updates_existing_level() -> TestResult {
    cases::level::set_updates_existing_level::<EngineBackend>().await
}

#[tokio::test]
async fn level_list_returns_empty_when_none_exist() -> TestResult {
    cases::level::list_returns_empty_when_none_exist::<EngineBackend>().await
}

#[tokio::test]
async fn level_list_returns_created_levels() -> TestResult {
    cases::level::list_returns_created_levels::<EngineBackend>().await
}

#[tokio::test]
async fn level_remove_makes_it_unlisted() -> TestResult {
    cases::level::remove_makes_it_unlisted::<EngineBackend>().await
}
#[tokio::test]
async fn level_set_prompt() -> TestResult {
    cases::level::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn level_show_prompt() -> TestResult {
    cases::level::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn level_list_prompt() -> TestResult {
    cases::level::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn level_remove_prompt() -> TestResult {
    cases::level::remove_prompt::<EngineBackend>().await
}

#[tokio::test]
async fn seed_core_creates_default_levels() -> TestResult {
    cases::seed::core_creates_default_levels::<EngineBackend>().await
}
#[tokio::test]
async fn seed_core_prompt() -> TestResult {
    cases::seed::core_prompt::<EngineBackend>().await
}

// Agent
#[tokio::test]
async fn agent_create_with_persona() -> TestResult {
    cases::agent::create_with_persona::<EngineBackend>().await
}
#[tokio::test]
async fn agent_show_returns_details() -> TestResult {
    cases::agent::show_returns_details::<EngineBackend>().await
}
#[tokio::test]
async fn agent_list_empty() -> TestResult {
    cases::agent::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn agent_list_populated() -> TestResult {
    cases::agent::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn agent_update_changes_fields() -> TestResult {
    cases::agent::update_changes_fields::<EngineBackend>().await
}
#[tokio::test]
async fn agent_remove_makes_it_unlisted() -> TestResult {
    cases::agent::remove_makes_it_unlisted::<EngineBackend>().await
}
#[tokio::test]
async fn agent_name_includes_persona_suffix() -> TestResult {
    cases::agent::name_includes_persona_suffix::<EngineBackend>().await
}
#[tokio::test]
async fn agent_create_prompt() -> TestResult {
    cases::agent::create_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn agent_show_prompt() -> TestResult {
    cases::agent::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn agent_list_prompt() -> TestResult {
    cases::agent::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn agent_update_prompt() -> TestResult {
    cases::agent::update_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn agent_remove_prompt() -> TestResult {
    cases::agent::remove_prompt::<EngineBackend>().await
}

// Connection
#[tokio::test]
async fn connection_create() -> TestResult {
    cases::connection::create::<EngineBackend>().await
}
#[tokio::test]
async fn connection_list_empty() -> TestResult {
    cases::connection::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn connection_list_populated() -> TestResult {
    cases::connection::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn connection_show_by_id() -> TestResult {
    cases::connection::show_by_id::<EngineBackend>().await
}
#[tokio::test]
async fn connection_remove_by_id() -> TestResult {
    cases::connection::remove_by_id::<EngineBackend>().await
}
#[tokio::test]
async fn connection_create_prompt_confirms_creation() -> TestResult {
    cases::connection::create_prompt_confirms_creation::<EngineBackend>().await
}
#[tokio::test]
async fn connection_show_prompt() -> TestResult {
    cases::connection::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn connection_list_prompt() -> TestResult {
    cases::connection::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn connection_remove_prompt() -> TestResult {
    cases::connection::remove_prompt::<EngineBackend>().await
}

// Cognition
#[tokio::test]
async fn cognition_add() -> TestResult {
    cases::cognition::add_creates_cognition::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_list_empty() -> TestResult {
    cases::cognition::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_list_populated() -> TestResult {
    cases::cognition::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_list_filters_by_agent() -> TestResult {
    cases::cognition::list_filters_by_agent::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_show_by_id() -> TestResult {
    cases::cognition::show_by_id::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_add_prompt_confirms_creation() -> TestResult {
    cases::cognition::add_prompt_confirms_creation::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_show_prompt() -> TestResult {
    cases::cognition::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_list_prompt() -> TestResult {
    cases::cognition::list_prompt::<EngineBackend>().await
}

// Memory
#[tokio::test]
async fn memory_add() -> TestResult {
    cases::memory::add_creates_memory::<EngineBackend>().await
}
#[tokio::test]
async fn memory_list_empty() -> TestResult {
    cases::memory::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn memory_list_populated() -> TestResult {
    cases::memory::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn memory_list_filters_by_agent() -> TestResult {
    cases::memory::list_filters_by_agent::<EngineBackend>().await
}
#[tokio::test]
async fn memory_show_by_id() -> TestResult {
    cases::memory::show_by_id::<EngineBackend>().await
}
#[tokio::test]
async fn memory_add_prompt_confirms_creation() -> TestResult {
    cases::memory::add_prompt_confirms_creation::<EngineBackend>().await
}
#[tokio::test]
async fn memory_show_prompt() -> TestResult {
    cases::memory::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn memory_list_prompt() -> TestResult {
    cases::memory::list_prompt::<EngineBackend>().await
}

// Experience
#[tokio::test]
async fn experience_create() -> TestResult {
    cases::experience::create::<EngineBackend>().await
}
#[tokio::test]
async fn experience_list_empty() -> TestResult {
    cases::experience::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn experience_list_populated() -> TestResult {
    cases::experience::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn experience_show_by_id() -> TestResult {
    cases::experience::show_by_id::<EngineBackend>().await
}
#[tokio::test]
async fn experience_update_description() -> TestResult {
    cases::experience::update_description::<EngineBackend>().await
}
#[tokio::test]
async fn experience_create_prompt_confirms_creation() -> TestResult {
    cases::experience::create_prompt_confirms_creation::<EngineBackend>().await
}
#[tokio::test]
async fn experience_show_prompt() -> TestResult {
    cases::experience::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn experience_list_prompt() -> TestResult {
    cases::experience::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn experience_update_prompt() -> TestResult {
    cases::experience::update_prompt::<EngineBackend>().await
}

// Lifecycle
#[tokio::test]
async fn lifecycle_wake() -> TestResult {
    cases::lifecycle::wake::<EngineBackend>().await
}
#[tokio::test]
async fn lifecycle_dream() -> TestResult {
    cases::lifecycle::dream::<EngineBackend>().await
}
#[tokio::test]
async fn lifecycle_introspect() -> TestResult {
    cases::lifecycle::introspect::<EngineBackend>().await
}
#[tokio::test]
async fn lifecycle_reflect() -> TestResult {
    cases::lifecycle::reflect::<EngineBackend>().await
}
#[tokio::test]
async fn lifecycle_sleep() -> TestResult {
    cases::lifecycle::sleep::<EngineBackend>().await
}
#[tokio::test]
async fn lifecycle_guidebook() -> TestResult {
    cases::lifecycle::guidebook::<EngineBackend>().await
}

// Lifecycle depth
#[tokio::test]
async fn dream_includes_vocabulary_and_connections() -> TestResult {
    cases::lifecycle::dream_includes_vocabulary_and_connections::<EngineBackend>().await
}

// Emerge / Recede
#[tokio::test]
async fn emerge_creates_and_wakes_agent() -> TestResult {
    cases::emerge::creates_and_wakes_agent::<EngineBackend>().await
}
#[tokio::test]
async fn recede_retires_agent() -> TestResult {
    cases::emerge::recede_retires_agent::<EngineBackend>().await
}

// Status
#[tokio::test]
async fn status_returns_agent_status() -> TestResult {
    cases::status::returns_agent_status::<EngineBackend>().await
}

// Doctor
#[tokio::test]
async fn doctor_reports_initialized() -> TestResult {
    cases::doctor::reports_initialized_system::<EngineBackend>().await
}
#[tokio::test]
async fn doctor_reports_uninitialized() -> TestResult {
    cases::doctor::reports_uninitialized_system::<EngineBackend>().await
}
#[tokio::test]
async fn doctor_prompt() -> TestResult {
    cases::doctor::doctor_prompt::<EngineBackend>().await
}

// Search
#[tokio::test]
async fn search_finds_cognition_content() -> TestResult {
    cases::search::finds_cognition_content::<EngineBackend>().await
}
#[tokio::test]
async fn search_finds_memory_content() -> TestResult {
    cases::search::finds_memory_content::<EngineBackend>().await
}
#[tokio::test]
async fn search_finds_experience_description() -> TestResult {
    cases::search::finds_experience_description::<EngineBackend>().await
}
#[tokio::test]
async fn search_finds_agent_description() -> TestResult {
    cases::search::finds_agent_description::<EngineBackend>().await
}
#[tokio::test]
async fn search_finds_persona_description() -> TestResult {
    cases::search::finds_persona_description::<EngineBackend>().await
}
#[tokio::test]
async fn search_returns_empty_for_no_match() -> TestResult {
    cases::search::returns_empty_for_no_match::<EngineBackend>().await
}
#[tokio::test]
async fn search_filters_by_agent() -> TestResult {
    cases::search::filters_by_agent::<EngineBackend>().await
}

// Search depth
#[tokio::test]
async fn search_finds_updated_agent_description() -> TestResult {
    cases::search::finds_updated_agent_description::<EngineBackend>().await
}
#[tokio::test]
async fn search_finds_updated_experience_description() -> TestResult {
    cases::search::finds_updated_experience_description::<EngineBackend>().await
}

// Import/Export depth
#[tokio::test]
async fn export_produces_file() -> TestResult {
    cases::import_export::export_produces_file::<EngineBackend>().await
}
#[tokio::test]
async fn import_restores_data() -> TestResult {
    cases::import_export::import_restores_data::<EngineBackend>().await
}
#[tokio::test]
async fn replay_rebuilds_projections() -> TestResult {
    cases::import_export::replay_rebuilds_projections::<EngineBackend>().await
}
#[tokio::test]
async fn export_import_preserves_storage() -> TestResult {
    cases::import_export::export_import_preserves_storage::<EngineBackend>().await
}

// Pressure
#[tokio::test]
async fn pressure_returns_readings() -> TestResult {
    cases::pressure::returns_readings_for_agent::<EngineBackend>().await
}
#[tokio::test]
async fn pressure_introspect_decreases_after_introspecting() -> TestResult {
    cases::pressure::introspect_pressure_decreases_after_introspecting::<EngineBackend>().await
}
#[tokio::test]
async fn pressure_catharsis_decreases_after_reflecting() -> TestResult {
    cases::pressure::catharsis_pressure_decreases_after_reflecting::<EngineBackend>().await
}

// Storage
#[tokio::test]
async fn storage_set_and_show() -> TestResult {
    cases::storage::set_and_show::<EngineBackend>().await
}
#[tokio::test]
async fn storage_list_empty() -> TestResult {
    cases::storage::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn storage_list_populated() -> TestResult {
    cases::storage::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn storage_remove() -> TestResult {
    cases::storage::remove::<EngineBackend>().await
}
#[tokio::test]
async fn storage_set_prompt() -> TestResult {
    cases::storage::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn storage_show_prompt() -> TestResult {
    cases::storage::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn storage_list_prompt() -> TestResult {
    cases::storage::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn storage_remove_prompt() -> TestResult {
    cases::storage::remove_prompt::<EngineBackend>().await
}

// Texture
#[tokio::test]
async fn texture_set_creates() -> TestResult {
    cases::texture::set_creates::<EngineBackend>().await
}
#[tokio::test]
async fn texture_set_updates() -> TestResult {
    cases::texture::set_updates::<EngineBackend>().await
}
#[tokio::test]
async fn texture_list_empty() -> TestResult {
    cases::texture::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn texture_list_populated() -> TestResult {
    cases::texture::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn texture_remove() -> TestResult {
    cases::texture::remove::<EngineBackend>().await
}
#[tokio::test]
async fn texture_set_prompt() -> TestResult {
    cases::texture::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn texture_show_prompt() -> TestResult {
    cases::texture::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn texture_list_prompt() -> TestResult {
    cases::texture::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn texture_remove_prompt() -> TestResult {
    cases::texture::remove_prompt::<EngineBackend>().await
}

// Sensation
#[tokio::test]
async fn sensation_set_creates() -> TestResult {
    cases::sensation::set_creates::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_set_updates() -> TestResult {
    cases::sensation::set_updates::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_list_empty() -> TestResult {
    cases::sensation::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_list_populated() -> TestResult {
    cases::sensation::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_remove() -> TestResult {
    cases::sensation::remove::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_set_prompt() -> TestResult {
    cases::sensation::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_show_prompt() -> TestResult {
    cases::sensation::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_list_prompt() -> TestResult {
    cases::sensation::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn sensation_remove_prompt() -> TestResult {
    cases::sensation::remove_prompt::<EngineBackend>().await
}

// Nature
#[tokio::test]
async fn nature_set_creates() -> TestResult {
    cases::nature::set_creates::<EngineBackend>().await
}
#[tokio::test]
async fn nature_set_updates() -> TestResult {
    cases::nature::set_updates::<EngineBackend>().await
}
#[tokio::test]
async fn nature_list_empty() -> TestResult {
    cases::nature::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn nature_list_populated() -> TestResult {
    cases::nature::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn nature_remove() -> TestResult {
    cases::nature::remove::<EngineBackend>().await
}
#[tokio::test]
async fn nature_set_prompt() -> TestResult {
    cases::nature::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn nature_show_prompt() -> TestResult {
    cases::nature::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn nature_list_prompt() -> TestResult {
    cases::nature::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn nature_remove_prompt() -> TestResult {
    cases::nature::remove_prompt::<EngineBackend>().await
}

// Persona
#[tokio::test]
async fn persona_set_creates() -> TestResult {
    cases::persona::set_creates::<EngineBackend>().await
}
#[tokio::test]
async fn persona_set_updates() -> TestResult {
    cases::persona::set_updates::<EngineBackend>().await
}
#[tokio::test]
async fn persona_list_empty() -> TestResult {
    cases::persona::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn persona_list_populated() -> TestResult {
    cases::persona::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn persona_remove() -> TestResult {
    cases::persona::remove::<EngineBackend>().await
}
#[tokio::test]
async fn persona_set_prompt() -> TestResult {
    cases::persona::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn persona_show_prompt() -> TestResult {
    cases::persona::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn persona_list_prompt() -> TestResult {
    cases::persona::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn persona_remove_prompt() -> TestResult {
    cases::persona::remove_prompt::<EngineBackend>().await
}

// Urge
#[tokio::test]
async fn urge_set_creates() -> TestResult {
    cases::urge::set_creates::<EngineBackend>().await
}
#[tokio::test]
async fn urge_set_updates() -> TestResult {
    cases::urge::set_updates::<EngineBackend>().await
}
#[tokio::test]
async fn urge_list_empty() -> TestResult {
    cases::urge::list_empty::<EngineBackend>().await
}
#[tokio::test]
async fn urge_list_populated() -> TestResult {
    cases::urge::list_populated::<EngineBackend>().await
}
#[tokio::test]
async fn urge_remove() -> TestResult {
    cases::urge::remove::<EngineBackend>().await
}
#[tokio::test]
async fn urge_set_prompt() -> TestResult {
    cases::urge::set_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn urge_show_prompt() -> TestResult {
    cases::urge::show_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn urge_list_prompt() -> TestResult {
    cases::urge::list_prompt::<EngineBackend>().await
}
#[tokio::test]
async fn urge_remove_prompt() -> TestResult {
    cases::urge::remove_prompt::<EngineBackend>().await
}

// Prompt output — lifecycle
#[tokio::test]
async fn prompt_dream_contains_identity() -> TestResult {
    cases::lifecycle::dream_prompt_contains_identity::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_dream_contains_vocabulary() -> TestResult {
    cases::lifecycle::dream_prompt_contains_vocabulary::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_dream_contains_memories() -> TestResult {
    cases::lifecycle::dream_prompt_contains_memories::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_dream_contains_cognitions() -> TestResult {
    cases::lifecycle::dream_prompt_contains_cognitions::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_introspect_contains_agent() -> TestResult {
    cases::lifecycle::introspect_prompt_contains_agent::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_reflect_contains_agent() -> TestResult {
    cases::lifecycle::reflect_prompt_contains_agent::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_guidebook_contains_capabilities() -> TestResult {
    cases::lifecycle::guidebook_prompt_contains_capabilities::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_wake_contains_identity() -> TestResult {
    cases::lifecycle::wake_prompt_contains_identity::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_sleep_contains_agent() -> TestResult {
    cases::lifecycle::sleep_prompt_contains_agent::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_sense_contains_agent() -> TestResult {
    cases::lifecycle::sense_prompt_contains_agent::<EngineBackend>().await
}

// Prompt output — emerge/recede
#[tokio::test]
async fn prompt_emerge_contains_identity() -> TestResult {
    cases::emerge::emerge_prompt_contains_identity::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_recede_contains_agent() -> TestResult {
    cases::emerge::recede_prompt_contains_agent::<EngineBackend>().await
}

// Prompt output — status
#[tokio::test]
async fn prompt_status_contains_agent() -> TestResult {
    cases::status::status_prompt_contains_agent::<EngineBackend>().await
}

// Prompt output — pressure
#[tokio::test]
async fn prompt_pressure_contains_readings() -> TestResult {
    cases::pressure::pressure_prompt_contains_readings::<EngineBackend>().await
}

// Prompt output — search
#[tokio::test]
async fn prompt_search_contains_results() -> TestResult {
    cases::search::search_prompt_contains_results::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_search_empty_results() -> TestResult {
    cases::search::search_prompt_empty_results::<EngineBackend>().await
}
