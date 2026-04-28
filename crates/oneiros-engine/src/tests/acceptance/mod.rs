use std::future::Future;
use std::time::{Duration, Instant};

use clap::Parser;

use crate::*;

pub(crate) type TestResult = Result<(), Box<dyn core::error::Error>>;

/// The execution contract — how commands get run against a backend.
///
/// Two execution modes mirror the CLI's output modes:
/// - `exec_json` returns typed data for structural assertions
/// - `exec_prompt` returns rendered prompt text for content assertions
///
/// Both backends must produce equivalent results — the engine directly,
/// the legacy by deserializing HTTP responses into the same types.
pub(crate) trait Backend: Sized {
    /// Create a new backend instance ready to execute commands.
    fn start() -> impl Future<Output = Result<Self, Box<dyn core::error::Error>>>;

    /// Start the service. Required before executing brain-scoped commands.
    fn start_service(&mut self) -> impl Future<Output = Result<(), Box<dyn core::error::Error>>>;

    /// Execute in JSON mode — returns typed data for structural assertions.
    fn exec_json(&self, command: &str) -> impl Future<Output = Result<Responses, Error>>;

    /// Execute in prompt mode — returns rendered text for content assertions.
    fn exec_prompt(&self, command: &str) -> impl Future<Output = Result<String, Error>>;
}

// ── Eventually-consistent assertions ────────────────────────────

/// Configuration for eventual-consistency retries.
#[derive(Clone)]
pub(crate) struct RetryPolicy {
    pub interval: Duration,
    pub timeout: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(50),
            timeout: Duration::from_secs(2),
        }
    }
}

/// A re-queryable handle for eventually-consistent assertions.
///
/// Does not hold a response — holds the ability to re-execute the command.
/// Each assertion method retries the query until the predicate passes or
/// the timeout expires.
///
/// Legacy backends pass on the first try (projections are inline).
/// Async backends may retry a few times while projections catch up.
pub(crate) struct Eventually<'h, B> {
    harness: &'h Harness<B>,
    command: String,
    policy: RetryPolicy,
}

impl<'h, B: Backend> Eventually<'h, B> {
    /// Assert on the JSON response, retrying until the predicate passes.
    ///
    /// Both execution errors (e.g., entity not found) and predicate failures
    /// are retried — the entity might not be projected yet.
    pub async fn assert_json<F>(self, predicate: F) -> TestResult
    where
        F: Fn(&Responses) -> Result<(), String>,
    {
        let deadline = Instant::now() + self.policy.timeout;
        #[expect(
            unused_assignments,
            reason = "We know we're overwriting the empty string"
        )]
        let mut last_err = String::new();

        loop {
            match self.harness.exec_json(&self.command).await {
                Ok(response) => match predicate(&response) {
                    Ok(()) => return Ok(()),
                    Err(msg) => last_err = msg,
                },
                Err(e) => last_err = e.to_string(),
            }

            if Instant::now() >= deadline {
                panic!(
                    "assertion not met within {:?} on command {:?}.\nLast failure: {last_err}",
                    self.policy.timeout, self.command,
                );
            }

            tokio::time::sleep(self.policy.interval).await;
        }
    }

    /// Assert on the prompt output, retrying until the predicate passes.
    pub async fn assert_prompt<F>(self, predicate: F) -> TestResult
    where
        F: Fn(&str) -> Result<(), String>,
    {
        let deadline = Instant::now() + self.policy.timeout;
        #[expect(
            unused_assignments,
            reason = "We know we're overwriting the empty string"
        )]
        let mut last_err = String::new();

        loop {
            match self.harness.exec_prompt(&self.command).await {
                Ok(prompt) => match predicate(&prompt) {
                    Ok(()) => return Ok(()),
                    Err(msg) => last_err = msg,
                },
                Err(e) => last_err = e.to_string(),
            }

            if Instant::now() >= deadline {
                panic!(
                    "assertion not met within {:?} on command {:?}.\nLast failure: {}",
                    self.policy.timeout, self.command, last_err
                );
            }

            tokio::time::sleep(self.policy.interval).await;
        }
    }

    /// Override the retry policy for this query.
    #[expect(
        dead_code,
        reason = "available for tests that need custom retry timing"
    )]
    pub fn with_policy(mut self, policy: RetryPolicy) -> Self {
        self.policy = policy;
        self
    }
}

/// Match on `response.data` and assert. Generates a predicate closure for
/// use with `Eventually::assert_json`.
///
/// ```ignore
/// // Match only — asserts the variant matches
/// harness.query("agent list").assert_json(expect!(
///     Responses::Agent(AgentResponse::Agents(AgentsResponse::V1(a))) if a.len() == 2
/// )).await
///
/// // Match with body — runs additional assertions on the extracted data
/// harness.query("agent show x").assert_json(expect!(
///     Responses::Agent(AgentResponse::AgentDetails(AgentDetailsResponse::V1(agent))) => {
///         assert_eq!(agent.name.as_str(), "x");
///     }
/// )).await
/// ```
macro_rules! expect {
    ($pattern:pat $(if $guard:expr)? => $body:block) => {
        |response: &Responses| {
            match response {
                $pattern $(if $guard)? => { $body; Ok(()) },
                other => Err(format!("expected {}, got {other:#?}", stringify!($pattern))),
            }
        }
    };
    ($pattern:pat $(if $guard:expr)?) => {
        |response: &Responses| {
            match response {
                $pattern $(if $guard)? => Ok(()),
                other => Err(format!("expected {}, got {other:#?}", stringify!($pattern))),
            }
        }
    };
}

// ── Harness ─────────────────────────────────────────────────────

/// The workflow language for tests — speaks user intent, not implementation.
///
/// Wraps a Backend and provides named workflow methods that compose
/// raw commands into meaningful setup steps. Test cases read like stories:
///
/// ```ignore
/// let harness = Harness::<MyBackend>::setup_system().await?;
/// let harness = harness.init_project().await?;
/// let response = harness.exec_json("level set session ...").await?;
/// ```
pub(crate) struct Harness<B> {
    backend: B,
}

impl<B: Backend> Harness<B> {
    /// Create a bare harness — no system, no project. System-scope tests start here.
    pub async fn started() -> Result<Self, Box<dyn core::error::Error>> {
        let backend = B::start().await?;
        Ok(Self { backend })
    }

    /// Initialize the host system. After this, system-scoped commands work
    /// (actor, brain, tenant, ticket).
    pub async fn setup_system() -> Result<Self, Box<dyn core::error::Error>> {
        let harness = Self::started().await?;
        harness
            .backend
            .exec_json("system init --name test --yes")
            .await?;
        Ok(harness)
    }

    /// Start the backing service explicitly. Most tests don't need this —
    /// `setup_system` starts the service automatically. Use for tests that
    /// need fine-grained control over sequencing.
    pub async fn start_service(mut self) -> Result<Self, Box<dyn core::error::Error>> {
        self.backend.start_service().await?;
        Ok(self)
    }

    /// Initialize a project brain. After this, brain-scoped commands work
    /// (agents, cognitions, memories, vocabulary, etc.)
    pub async fn init_project() -> Result<Self, Box<dyn core::error::Error>> {
        let harness = Self::setup_system().await?.start_service().await?;
        harness
            .backend
            .exec_json("project init --yes")
            .await
            .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
        Ok(harness)
    }

    /// Initialize a project and seed it with core vocabulary.
    pub async fn seed_project() -> Result<Self, Box<dyn core::error::Error>> {
        let harness = Self::init_project().await?;
        harness
            .backend
            .exec_json("seed core")
            .await
            .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
        Ok(harness)
    }

    /// Execute in JSON mode — delegates to the backend.
    ///
    /// Use for write operations where you assert on the immediate response,
    /// or for setup commands that don't need eventual-consistency handling.
    pub async fn exec_json(&self, command: &str) -> Result<Responses, Error> {
        self.backend.exec_json(command).await
    }

    /// Execute in prompt mode — delegates to the backend.
    pub async fn exec_prompt(&self, command: &str) -> Result<String, Error> {
        self.backend.exec_prompt(command).await
    }

    /// Start an eventually-consistent query.
    ///
    /// The returned handle re-executes the command on each assertion attempt
    /// until the assertion passes or the timeout expires. Use this for
    /// read-after-write assertions where projections might not have caught up.
    ///
    /// ```ignore
    /// harness.exec_json("agent create viewer process ...").await?;
    /// harness.query("agent show viewer.process").assert_json(expect!(
    ///     Responses::Agent(AgentResponse::AgentDetails(_))
    /// )).await
    /// ```
    pub fn query(&self, command: &str) -> Eventually<'_, B> {
        Eventually {
            harness: self,
            command: command.to_string(),
            policy: RetryPolicy::default(),
        }
    }
}

// ── Cases ───────────────────────────────────────────────────────

mod cases;

// ── Engine backend ──────────────────────────────────────────────

struct EngineBackend {
    engine: Engine,
    _dir: tempfile::TempDir,
    _server: Option<ServerHandle>,
}

impl Backend for EngineBackend {
    async fn start() -> Result<Self, Box<dyn core::error::Error>> {
        let dir = tempfile::TempDir::new()?;
        let config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .brain(BrainName::new("test-project"))
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse()?)
                    .build(),
            )
            .build();

        let mut engine = Engine::new(config);

        // Start the HTTP server eagerly — the engine CLI routes all
        // commands through HTTP clients, so the service must be running
        // before any commands execute. start() resolves the ephemeral
        // port and updates the engine's config automatically.
        let handle = engine.start().await?;

        Ok(Self {
            engine,
            _dir: dir,
            _server: Some(handle),
        })
    }

    async fn exec_json(&self, command: &str) -> Result<Responses, Error> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        let cli = Cli::try_parse_from(&full_args).map_err(|e| Error::Context(e.to_string()))?;
        let rendered = cli.execute(self.engine.config()).await?;

        Ok(rendered.into_response())
    }

    async fn exec_prompt(&self, command: &str) -> Result<String, Error> {
        let args = shell_words(command);
        let mut full_args = vec!["oneiros".to_string()];
        full_args.extend(args);

        let cli = Cli::try_parse_from(&full_args).map_err(|e| Error::Context(e.to_string()))?;
        let rendered = cli.execute(self.engine.config()).await?;

        Ok(rendered.prompt().to_string())
    }

    async fn start_service(&mut self) -> Result<(), Box<dyn core::error::Error>> {
        // Server is started eagerly in start() — this is a no-op.
        Ok(())
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

// ── Test registrations ──────────────────────────────────────────

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
async fn agent_show_by_ref() -> TestResult {
    cases::agent::show_by_ref::<EngineBackend>().await
}
#[tokio::test]
async fn agent_show_by_wrong_kind_ref_errors() -> TestResult {
    cases::agent::show_by_wrong_kind_ref_errors::<EngineBackend>().await
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
async fn cognition_show_json_includes_ref() -> TestResult {
    cases::cognition::show_json_includes_ref::<EngineBackend>().await
}
#[tokio::test]
async fn cognition_list_json_items_include_refs() -> TestResult {
    cases::cognition::list_json_items_include_refs::<EngineBackend>().await
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
async fn experience_show_by_ref() -> TestResult {
    cases::experience::show_by_ref::<EngineBackend>().await
}
#[tokio::test]
async fn experience_show_by_wrong_kind_ref_errors() -> TestResult {
    cases::experience::show_by_wrong_kind_ref_errors::<EngineBackend>().await
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
async fn status_returns_activity_table_json() -> TestResult {
    cases::status::returns_activity_table::<EngineBackend>().await
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
#[tokio::test]
async fn import_bootstraps_fresh_brain() -> TestResult {
    cases::import_export::import_bootstraps_fresh_brain::<EngineBackend>().await
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

// Greeting rendering
#[tokio::test]
async fn prompt_dream_omits_vocabulary() -> TestResult {
    cases::lifecycle::dream_prompt_omits_vocabulary::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_dream_omits_non_core_memories() -> TestResult {
    cases::lifecycle::dream_prompt_omits_non_core_memories::<EngineBackend>().await
}

// Prompt output — lifecycle
#[tokio::test]
async fn prompt_dream_contains_identity() -> TestResult {
    cases::lifecycle::dream_prompt_contains_identity::<EngineBackend>().await
}
#[tokio::test]
async fn prompt_dream_contains_continuity() -> TestResult {
    cases::lifecycle::dream_prompt_contains_continuity::<EngineBackend>().await
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
async fn prompt_status_shows_agents() -> TestResult {
    cases::status::status_shows_agents::<EngineBackend>().await
}

#[tokio::test]
async fn status_returns_activity_table() -> TestResult {
    cases::status::returns_activity_table::<EngineBackend>().await
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
