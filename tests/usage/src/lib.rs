use std::future::Future;

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

    /// Start the backing service. Required between system init and project init
    /// for backends that route through HTTP.
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
}
