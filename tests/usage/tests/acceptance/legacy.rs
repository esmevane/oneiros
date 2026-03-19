use clap::Parser;
use oneiros_cli::Context;
use oneiros_config::{Config, ServiceConfig};
use oneiros_detect_project_name::ProjectRoot;
use oneiros_http::HttpService;
use oneiros_usage::*;

use crate::cases;

pub struct Legacy {
    _temp: tempfile::TempDir,
    context: Context,
    server: Option<tokio::task::JoinHandle<()>>,
}

impl Backend for Legacy {
    async fn start() -> Result<Self, Box<dyn core::error::Error>> {
        let temp = tempfile::TempDir::new()?;
        let data_dir = temp.path().join("data");
        let config_dir = temp.path().join("config");

        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&config_dir)?;

        let context = Context::builder()
            .data_dir(data_dir)
            .config_dir(config_dir)
            .build();

        Ok(Self {
            _temp: temp,
            context,
            server: None,
        })
    }

    async fn exec(&self, command: &str) -> Result<serde_json::Value, Box<dyn core::error::Error>> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros"];
        full_args.extend(args.iter().map(String::as_str));

        let cli = oneiros_cli::Cli::try_parse_from(full_args)?;
        let result = cli.run_with(&self.context).await?;

        let values: Vec<serde_json::Value> = result
            .outcomes
            .into_iter()
            .filter_map(|outcome| serde_json::to_value(outcome).ok())
            .collect();

        Ok(serde_json::Value::Array(values))
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        let project_root = self._temp.path().join("project");
        std::fs::create_dir_all(&project_root)?;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        self.context = Context::builder()
            .data_dir(self.context.data_dir().to_path_buf())
            .config_dir(self.context.config_dir().to_path_buf())
            .project(ProjectRoot::new("test-project", project_root))
            .config(
                Config::builder()
                    .service(ServiceConfig::builder().port(addr.port()).build())
                    .build(),
            )
            .build();

        let http = HttpService::init(self.context.clone())?;

        self.server = Some(tokio::spawn(async move {
            http.serve(listener).await.unwrap();
        }));

        Ok(())
    }
}

impl Drop for Legacy {
    fn drop(&mut self) {
        if let Some(handle) = &self.server {
            handle.abort();
        }
    }
}

/// Split a command string into words, respecting single and double quotes.
fn shell_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quote: Option<char> = None;

    while let Some(ch) = chars.next() {
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
    cases::system::init_creates_tenant_and_actor::<Legacy>().await
}

#[tokio::test]
async fn system_init_is_idempotent() -> TestResult {
    cases::system::init_is_idempotent::<Legacy>().await
}

#[tokio::test]
async fn project_init_creates_brain() -> TestResult {
    cases::project::init_creates_brain::<Legacy>().await
}

#[tokio::test]
async fn level_set_creates_a_new_level() -> TestResult {
    cases::level::set_creates_a_new_level::<Legacy>().await
}

#[tokio::test]
async fn level_set_updates_existing_level() -> TestResult {
    cases::level::set_updates_existing_level::<Legacy>().await
}

#[tokio::test]
async fn level_list_returns_empty_when_none_exist() -> TestResult {
    cases::level::list_returns_empty_when_none_exist::<Legacy>().await
}

#[tokio::test]
async fn level_list_returns_created_levels() -> TestResult {
    cases::level::list_returns_created_levels::<Legacy>().await
}

#[tokio::test]
async fn level_remove_makes_it_unlisted() -> TestResult {
    cases::level::remove_makes_it_unlisted::<Legacy>().await
}

#[tokio::test]
async fn seed_core_creates_default_levels() -> TestResult {
    cases::seed::core_creates_default_levels::<Legacy>().await
}

// Agent
#[tokio::test]
async fn agent_create_with_persona() -> TestResult {
    cases::agent::create_with_persona::<Legacy>().await
}
#[tokio::test]
async fn agent_show_returns_details() -> TestResult {
    cases::agent::show_returns_details::<Legacy>().await
}
#[tokio::test]
async fn agent_list_empty() -> TestResult {
    cases::agent::list_empty::<Legacy>().await
}
#[tokio::test]
async fn agent_list_populated() -> TestResult {
    cases::agent::list_populated::<Legacy>().await
}
#[tokio::test]
async fn agent_update_changes_fields() -> TestResult {
    cases::agent::update_changes_fields::<Legacy>().await
}
#[tokio::test]
async fn agent_remove_makes_it_unlisted() -> TestResult {
    cases::agent::remove_makes_it_unlisted::<Legacy>().await
}
#[tokio::test]
async fn agent_name_includes_persona_suffix() -> TestResult {
    cases::agent::name_includes_persona_suffix::<Legacy>().await
}

// Connection
#[tokio::test]
async fn connection_create() -> TestResult {
    cases::connection::create::<Legacy>().await
}
#[tokio::test]
async fn connection_list_empty() -> TestResult {
    cases::connection::list_empty::<Legacy>().await
}
#[tokio::test]
async fn connection_list_populated() -> TestResult {
    cases::connection::list_populated::<Legacy>().await
}
#[tokio::test]
async fn connection_show_by_id() -> TestResult {
    cases::connection::show_by_id::<Legacy>().await
}
#[tokio::test]
async fn connection_remove_by_id() -> TestResult {
    cases::connection::remove_by_id::<Legacy>().await
}

// Experience
#[tokio::test]
async fn experience_create() -> TestResult {
    cases::experience::create::<Legacy>().await
}
#[tokio::test]
async fn experience_list_empty() -> TestResult {
    cases::experience::list_empty::<Legacy>().await
}
#[tokio::test]
async fn experience_list_populated() -> TestResult {
    cases::experience::list_populated::<Legacy>().await
}
#[tokio::test]
async fn experience_show_by_id() -> TestResult {
    cases::experience::show_by_id::<Legacy>().await
}
#[tokio::test]
async fn experience_update_description() -> TestResult {
    cases::experience::update_description::<Legacy>().await
}

// Cognition
#[tokio::test]
async fn cognition_add() -> TestResult {
    cases::cognition::add_creates_cognition::<Legacy>().await
}
#[tokio::test]
async fn cognition_list_empty() -> TestResult {
    cases::cognition::list_empty::<Legacy>().await
}
#[tokio::test]
async fn cognition_list_populated() -> TestResult {
    cases::cognition::list_populated::<Legacy>().await
}
#[tokio::test]
async fn cognition_list_filters_by_agent() -> TestResult {
    cases::cognition::list_filters_by_agent::<Legacy>().await
}
#[tokio::test]
async fn cognition_show_by_id() -> TestResult {
    cases::cognition::show_by_id::<Legacy>().await
}

// Memory
#[tokio::test]
async fn memory_add() -> TestResult {
    cases::memory::add_creates_memory::<Legacy>().await
}
#[tokio::test]
async fn memory_list_empty() -> TestResult {
    cases::memory::list_empty::<Legacy>().await
}
#[tokio::test]
async fn memory_list_populated() -> TestResult {
    cases::memory::list_populated::<Legacy>().await
}
#[tokio::test]
async fn memory_list_filters_by_agent() -> TestResult {
    cases::memory::list_filters_by_agent::<Legacy>().await
}
#[tokio::test]
async fn memory_show_by_id() -> TestResult {
    cases::memory::show_by_id::<Legacy>().await
}

// Search
#[tokio::test]
async fn search_finds_cognition_content() -> TestResult {
    cases::search::finds_cognition_content::<Legacy>().await
}
#[tokio::test]
async fn search_finds_memory_content() -> TestResult {
    cases::search::finds_memory_content::<Legacy>().await
}
#[tokio::test]
async fn search_finds_experience_description() -> TestResult {
    cases::search::finds_experience_description::<Legacy>().await
}
#[tokio::test]
async fn search_finds_agent_description() -> TestResult {
    cases::search::finds_agent_description::<Legacy>().await
}
#[tokio::test]
async fn search_finds_persona_description() -> TestResult {
    cases::search::finds_persona_description::<Legacy>().await
}
#[tokio::test]
async fn search_returns_empty_for_no_match() -> TestResult {
    cases::search::returns_empty_for_no_match::<Legacy>().await
}
#[tokio::test]
async fn search_filters_by_agent() -> TestResult {
    cases::search::filters_by_agent::<Legacy>().await
}

// Pressure
#[tokio::test]
async fn pressure_returns_readings() -> TestResult {
    cases::pressure::returns_readings_for_agent::<Legacy>().await
}

// Storage
#[tokio::test]
async fn storage_set_and_show() -> TestResult {
    cases::storage::set_and_show::<Legacy>().await
}
#[tokio::test]
async fn storage_list_empty() -> TestResult {
    cases::storage::list_empty::<Legacy>().await
}
#[tokio::test]
async fn storage_list_populated() -> TestResult {
    cases::storage::list_populated::<Legacy>().await
}
#[tokio::test]
async fn storage_remove() -> TestResult {
    cases::storage::remove::<Legacy>().await
}

// Lifecycle
#[tokio::test]
async fn lifecycle_wake() -> TestResult {
    cases::lifecycle::wake::<Legacy>().await
}
#[tokio::test]
async fn lifecycle_dream() -> TestResult {
    cases::lifecycle::dream::<Legacy>().await
}
#[tokio::test]
async fn lifecycle_introspect() -> TestResult {
    cases::lifecycle::introspect::<Legacy>().await
}
#[tokio::test]
async fn lifecycle_reflect() -> TestResult {
    cases::lifecycle::reflect::<Legacy>().await
}
#[tokio::test]
async fn lifecycle_sleep() -> TestResult {
    cases::lifecycle::sleep::<Legacy>().await
}
#[tokio::test]
async fn lifecycle_guidebook() -> TestResult {
    cases::lifecycle::guidebook::<Legacy>().await
}

// Emerge / Recede
#[tokio::test]
async fn emerge_creates_and_wakes_agent() -> TestResult {
    cases::emerge::creates_and_wakes_agent::<Legacy>().await
}
#[tokio::test]
async fn recede_retires_agent() -> TestResult {
    cases::emerge::recede_retires_agent::<Legacy>().await
}

// Status
#[tokio::test]
async fn status_returns_agent_status() -> TestResult {
    cases::status::returns_agent_status::<Legacy>().await
}

// Event
#[tokio::test]
async fn event_list_shows_events() -> TestResult {
    cases::event::list_shows_events::<Legacy>().await
}

// Texture
#[tokio::test]
async fn texture_set_creates() -> TestResult {
    cases::texture::set_creates::<Legacy>().await
}
#[tokio::test]
async fn texture_set_updates() -> TestResult {
    cases::texture::set_updates::<Legacy>().await
}
#[tokio::test]
async fn texture_list_empty() -> TestResult {
    cases::texture::list_empty::<Legacy>().await
}
#[tokio::test]
async fn texture_list_populated() -> TestResult {
    cases::texture::list_populated::<Legacy>().await
}
#[tokio::test]
async fn texture_remove() -> TestResult {
    cases::texture::remove::<Legacy>().await
}

// Sensation
#[tokio::test]
async fn sensation_set_creates() -> TestResult {
    cases::sensation::set_creates::<Legacy>().await
}
#[tokio::test]
async fn sensation_set_updates() -> TestResult {
    cases::sensation::set_updates::<Legacy>().await
}
#[tokio::test]
async fn sensation_list_empty() -> TestResult {
    cases::sensation::list_empty::<Legacy>().await
}
#[tokio::test]
async fn sensation_list_populated() -> TestResult {
    cases::sensation::list_populated::<Legacy>().await
}
#[tokio::test]
async fn sensation_remove() -> TestResult {
    cases::sensation::remove::<Legacy>().await
}

// Nature
#[tokio::test]
async fn nature_set_creates() -> TestResult {
    cases::nature::set_creates::<Legacy>().await
}
#[tokio::test]
async fn nature_set_updates() -> TestResult {
    cases::nature::set_updates::<Legacy>().await
}
#[tokio::test]
async fn nature_list_empty() -> TestResult {
    cases::nature::list_empty::<Legacy>().await
}
#[tokio::test]
async fn nature_list_populated() -> TestResult {
    cases::nature::list_populated::<Legacy>().await
}
#[tokio::test]
async fn nature_remove() -> TestResult {
    cases::nature::remove::<Legacy>().await
}

// Persona
#[tokio::test]
async fn persona_set_creates() -> TestResult {
    cases::persona::set_creates::<Legacy>().await
}
#[tokio::test]
async fn persona_set_updates() -> TestResult {
    cases::persona::set_updates::<Legacy>().await
}
#[tokio::test]
async fn persona_list_empty() -> TestResult {
    cases::persona::list_empty::<Legacy>().await
}
#[tokio::test]
async fn persona_list_populated() -> TestResult {
    cases::persona::list_populated::<Legacy>().await
}
#[tokio::test]
async fn persona_remove() -> TestResult {
    cases::persona::remove::<Legacy>().await
}

// Urge
#[tokio::test]
async fn urge_set_creates() -> TestResult {
    cases::urge::set_creates::<Legacy>().await
}
#[tokio::test]
async fn urge_set_updates() -> TestResult {
    cases::urge::set_updates::<Legacy>().await
}
#[tokio::test]
async fn urge_list_empty() -> TestResult {
    cases::urge::list_empty::<Legacy>().await
}
#[tokio::test]
async fn urge_list_populated() -> TestResult {
    cases::urge::list_populated::<Legacy>().await
}
#[tokio::test]
async fn urge_remove() -> TestResult {
    cases::urge::remove::<Legacy>().await
}
