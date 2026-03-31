use std::future::Future;
use std::time::{Duration, Instant};

pub type TestResult = Result<(), Box<dyn core::error::Error>>;

/// The execution contract — how commands get run against a backend.
///
/// Two execution modes mirror the CLI's output modes:
/// - `exec_json` returns typed data for structural assertions
/// - `exec_prompt` returns rendered prompt text for content assertions
///
/// Both backends must produce equivalent results — the engine directly,
/// the legacy by deserializing HTTP responses into the same types.
pub trait Backend: Sized {
    /// Create a new backend instance ready to execute commands.
    fn start() -> impl Future<Output = Result<Self, Box<dyn core::error::Error>>>;

    /// Start the service. Required before executing brain-scoped commands.
    fn start_service(&mut self) -> impl Future<Output = Result<(), Box<dyn core::error::Error>>>;

    /// Execute in JSON mode — returns typed data for structural assertions.
    fn exec_json(
        &self,
        command: &str,
    ) -> impl Future<
        Output = Result<oneiros_engine::Response<oneiros_engine::Responses>, oneiros_engine::Error>,
    >;

    /// Execute in prompt mode — returns rendered text for content assertions.
    fn exec_prompt(
        &self,
        command: &str,
    ) -> impl Future<Output = Result<String, oneiros_engine::Error>>;
}

// ── Eventually-consistent assertions ────────────────────────────

/// Configuration for eventual-consistency retries.
#[derive(Clone)]
pub struct RetryPolicy {
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
pub struct Eventually<'h, B> {
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
        F: Fn(&oneiros_engine::Response<oneiros_engine::Responses>) -> Result<(), String>,
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
///     Responses::Agent(AgentResponse::Agents(a)) if a.len() == 2
/// )).await
///
/// // Match with body — runs additional assertions on the extracted data
/// harness.query("agent show x").assert_json(expect!(
///     Responses::Agent(AgentResponse::AgentDetails(agent)) => {
///         assert_eq!(agent.name.as_str(), "x");
///     }
/// )).await
/// ```
#[macro_export]
macro_rules! expect {
    ($pattern:pat $(if $guard:expr)? => $body:block) => {
        |response: &oneiros_engine::Response<oneiros_engine::Responses>| {
            match &response.data {
                $pattern $(if $guard)? => { $body; Ok(()) },
                other => Err(format!("expected {}, got {other:#?}", stringify!($pattern))),
            }
        }
    };
    ($pattern:pat $(if $guard:expr)?) => {
        |response: &oneiros_engine::Response<oneiros_engine::Responses>| {
            match &response.data {
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
pub struct Harness<B> {
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
            .await
            .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
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
    pub async fn exec_json(
        &self,
        command: &str,
    ) -> Result<oneiros_engine::Response<oneiros_engine::Responses>, oneiros_engine::Error> {
        self.backend.exec_json(command).await
    }

    /// Execute in prompt mode — delegates to the backend.
    pub async fn exec_prompt(&self, command: &str) -> Result<String, oneiros_engine::Error> {
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
