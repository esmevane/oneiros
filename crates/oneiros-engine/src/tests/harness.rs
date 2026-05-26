//! Test harness — a first-class testing surface for the engine.
//!
//! `TestApp` owns the full lifecycle: config, server, and teardown.
//! Tests interact through the same layers the real host uses:
//! CLI commands go through clap → client → HTTP → service → event log.
//!
//! ```rust,ignore
//! let app = TestApp::new()
//!     .await?
//!     .init_host()
//!     .await?
//!     .init_project()
//!     .await?
//!     .seed_core()
//!     .await?;
//!
//! app.command("agent create thinker process").await?;
//!
//! let client = app.client();
//! let agent = client.agent().get(&GetAgent::V1(...)).await?;
//! ```

use clap::Parser;

use crate::*;

/// A running test application with an isolated data directory and server.
///
/// Every `TestApp` gets its own tempdir and ephemeral-port server.
/// Commands and client calls go through the full HTTP stack.
pub(crate) struct TestApp {
    engine: Engine,
    _dir: tempfile::TempDir,
    _handle: ServerHandle,
}

impl TestApp {
    /// Boot a new test app: tempdir, config, server on a random port.
    pub(crate) async fn new() -> Result<Self, Box<dyn core::error::Error>> {
        let dir = tempfile::tempdir().expect("create tempdir");
        Self::with_data_dir(dir).await
    }

    /// Boot a test app reusing an externally-staged tempdir. Used by
    /// migration tests that pre-write old-layout files into the data-dir
    /// and need the server to boot against that exact directory (so the
    /// production migration hook runs).
    pub(crate) async fn with_data_dir(
        dir: tempfile::TempDir,
    ) -> Result<Self, Box<dyn core::error::Error>> {
        let mut config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .project(ProjectName::new("test"))
            .output(OutputMode::Json)
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse().unwrap())
                    .build(),
            )
            // Tighter fetch window for tests — patience semantics still
            // exercised, but missing-entity tests don't pay the production
            // 2-second wait per call.
            .fetch(Fetch {
                interval: std::time::Duration::from_millis(2),
                timeout: std::time::Duration::from_millis(100),
            })
            .build();

        let handle = Server::new(config.clone()).spawn().await?;
        config.service.address = handle.address();

        // Wait for the server to finish booting (HostKey::ensure, DB setup)
        // before any test makes requests. Otherwise Client::from_config
        // races against HostKey creation and falls back to anonymous auth.
        wait_for_server(&config).await?;

        let engine = Engine::new(config);

        Ok(Self {
            engine,
            _dir: dir,
            _handle: handle,
        })
    }

    /// Initialize the host database and default tenant/actor.
    pub(crate) async fn init_host(self) -> Result<Self, Box<dyn core::error::Error>> {
        self.command("host init --name test").await?;
        Ok(self)
    }

    /// Create a project with the default name.
    pub(crate) async fn init_project(self) -> Result<Self, Box<dyn core::error::Error>> {
        let project = self.engine.config().project.to_string();
        self.command(&format!("project create --name {project}"))
            .await?;
        Ok(self)
    }

    /// Seed the core vocabulary (textures, levels, sensations, etc.).
    pub(crate) async fn seed_core(self) -> Result<Self, Box<dyn core::error::Error>> {
        self.command("seed core").await?;
        Ok(self)
    }

    /// Execute a CLI command string through the full stack.
    ///
    /// The command is parsed through clap, dispatched through the engine,
    /// and exercised against the running server. This is the same path
    /// the real binary takes.
    ///
    /// ```rust,ignore
    /// app.command("agent create thinker process").await?;
    /// app.command("persona show process").await?;
    /// ```
    pub(crate) async fn command(
        &self,
        input: &str,
    ) -> Result<Rendered<Responses>, Box<dyn core::error::Error>> {
        // Build the arg list: "oneiros" + the user's command tokens.
        // Prepend global flags so clap routes to the right config.
        let config = self.engine.config();
        let mut args: Vec<String> = vec![
            "oneiros".into(),
            "--data-dir".into(),
            config.data_dir.display().to_string(),
            "--project".into(),
            config.project.to_string(),
        ];
        args.extend(
            shlex::split(input)
                .unwrap_or_else(|| panic!("invalid shell words in command: {input}")),
        );

        let cli = Cli::try_parse_from(&args)?;

        // Execute against our engine's config (which has the correct port).
        Ok(self.engine.execute(&cli).await?)
    }

    /// Get a typed HTTP client connected to this app's server.
    ///
    /// The client authenticates with the project's token (written during
    /// `init_project`). Domain-specific clients are available via methods
    /// like `.persona()`, `.agent()`, etc.
    pub(crate) fn client(&self) -> TestClient {
        let client = Client::from_config(self.engine.config()).expect("test client");
        TestClient { client }
    }

    /// The engine config — useful for tests that need the underlying
    /// ProjectLog alongside the HTTP stack.
    pub(crate) fn config(&self) -> &Config {
        self.engine.config()
    }

    /// The base URL of the running server (e.g. `http://127.0.0.1:PORT`).
    pub(crate) fn base_url(&self) -> String {
        self.engine.config().base_url()
    }

    /// The MCP endpoint URL (e.g. `http://127.0.0.1:PORT/mcp`).
    pub(crate) fn mcp_url(&self) -> String {
        format!("{}/mcp", self.base_url())
    }

    /// The project token, if one exists (written during `init_project`).
    pub(crate) fn token(&self) -> Option<Token> {
        self.engine.config().token()
    }
}

/// A typed HTTP client for test assertions.
///
/// Wraps the shared `Client` and exposes per-domain helpers. Each accessor
/// returns a borrowing wrapper (e.g. `AgentClient<'a>`) that knows how to
/// dispatch typed `ClientRequest`s and decode the response bytes back into
/// the matching response shape.
///
/// Production code dispatches typed requests via `ClientRequest::execute_request`
/// directly — these wrappers exist only for tests, and only carry the methods
/// the suite actually exercises. Add to a wrapper here when a workflow needs
/// a new shape; do not let test ergonomics leak into the production client.
pub(crate) struct TestClient {
    client: Client,
}

impl TestClient {
    pub(crate) fn persona(&self) -> PersonaClient<'_> {
        PersonaClient::new(&self.client)
    }

    pub(crate) fn agent(&self) -> AgentClient<'_> {
        AgentClient::new(&self.client)
    }

    pub(crate) fn level(&self) -> LevelClient<'_> {
        LevelClient::new(&self.client)
    }

    pub(crate) fn texture(&self) -> TextureClient<'_> {
        TextureClient::new(&self.client)
    }

    pub(crate) fn sensation(&self) -> SensationClient<'_> {
        SensationClient::new(&self.client)
    }

    pub(crate) fn nature(&self) -> NatureClient<'_> {
        NatureClient::new(&self.client)
    }

    pub(crate) fn urge(&self) -> UrgeClient<'_> {
        UrgeClient::new(&self.client)
    }

    pub(crate) fn cognition(&self) -> CognitionClient<'_> {
        CognitionClient::new(&self.client)
    }

    pub(crate) fn memory(&self) -> MemoryClient<'_> {
        MemoryClient::new(&self.client)
    }

    pub(crate) fn experience(&self) -> ExperienceClient<'_> {
        ExperienceClient::new(&self.client)
    }

    pub(crate) fn connection(&self) -> ConnectionClient<'_> {
        ConnectionClient::new(&self.client)
    }

    pub(crate) fn storage(&self) -> StorageClient<'_> {
        StorageClient::new(&self.client)
    }

    pub(crate) fn continuity(&self) -> ContinuityClient<'_> {
        ContinuityClient::new(&self.client)
    }

    pub(crate) fn search(&self) -> SearchClient<'_> {
        SearchClient::new(&self.client)
    }

    pub(crate) fn trail(&self) -> TrailClient<'_> {
        TrailClient::new(&self.client)
    }

    pub(crate) fn pressure(&self) -> PressureClient<'_> {
        PressureClient::new(&self.client)
    }

    pub(crate) fn tenant(&self) -> TenantClient<'_> {
        TenantClient::new(&self.client)
    }

    pub(crate) fn actor(&self) -> ActorClient<'_> {
        ActorClient::new(&self.client)
    }

    pub(crate) fn project(&self) -> ProjectClient<'_> {
        ProjectClient::new(&self.client)
    }

    pub(crate) fn ticket(&self) -> TicketClient<'_> {
        TicketClient::new(&self.client)
    }

    pub(crate) fn bookmark(&self) -> BookmarkClient<'_> {
        BookmarkClient::new(&self.client)
    }
}

/// Decode bytes from the shared client into a typed response. The shared
/// `Client` returns raw bytes; each test wrapper turns those into the
/// domain's response enum and surfaces serde failures as
/// `ClientError::InvalidRequest` (the only string-bearing variant we have).
fn decode<R: serde::de::DeserializeOwned>(bytes: Vec<u8>, domain: &str) -> Result<R, ClientError> {
    serde_json::from_slice(&bytes)
        .map_err(|error| ClientError::InvalidRequest(format!("{domain} response: {error}")))
}

// ---------------------------------------------------------------------------
// Per-domain test wrappers.
//
// One borrowing wrapper per domain. Methods are added when a test reaches for
// them; production CLI/MCP/HTTP layers dispatch `ClientRequest::execute_request`
// directly without going through these.
// ---------------------------------------------------------------------------

pub(crate) struct AgentClient<'a> {
    client: &'a Client,
}

impl<'a> AgentClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(&self, request: &CreateAgent) -> Result<AgentResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "agent")
    }

    pub(crate) async fn get(&self, request: &GetAgent) -> Result<AgentResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "agent")
    }

    pub(crate) async fn list(&self, request: &ListAgents) -> Result<AgentResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "agent")
    }
}

pub(crate) struct CognitionClient<'a> {
    client: &'a Client,
}

impl<'a> CognitionClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn add(
        &self,
        request: &AddCognition,
    ) -> Result<CognitionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "cognition")
    }

    pub(crate) async fn list(
        &self,
        request: &ListCognitions,
    ) -> Result<CognitionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "cognition")
    }
}

pub(crate) struct ContinuityClient<'a> {
    client: &'a Client,
}

impl<'a> ContinuityClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn emerge(
        &self,
        request: &EmergeAgent,
    ) -> Result<ContinuityResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn dream(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.dream_with(agent, &DreamOverrides::default()).await
    }

    pub(crate) async fn dream_with(
        &self,
        agent: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ClientError> {
        // The `DreamAgent` request hard-codes `DreamOverrides::default()`
        // when serializing to a URL; tests that want explicit overrides
        // build the query string here and call the raw client.
        let query = encode_dream_overrides(overrides);
        let path = if query.is_empty() {
            format!("/continuity/{agent}/dream")
        } else {
            format!("/continuity/{agent}/dream?{query}")
        };
        let bytes = self.client.post(&path, &serde_json::Value::Null).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn introspect(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        let request: IntrospectAgent = IntrospectAgent::builder_v1()
            .agent(agent.clone())
            .build()
            .into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn reflect(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        let request: ReflectAgent = ReflectAgent::builder_v1()
            .agent(agent.clone())
            .build()
            .into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn sleep(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        let request: SleepAgent = SleepAgent::builder_v1().agent(agent.clone()).build().into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn recede(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        let request: RecedeAgent = RecedeAgent::builder_v1()
            .agent(agent.clone())
            .build()
            .into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn guidebook(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        let request: GuidebookAgent = GuidebookAgent::builder_v1()
            .agent(agent.clone())
            .build()
            .into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }

    pub(crate) async fn status(&self) -> Result<ContinuityResponse, ClientError> {
        let request: StatusAgent = StatusAgent::builder_v1().build().into();
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "continuity")
    }
}

fn encode_dream_overrides(overrides: &DreamOverrides) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(value) = overrides.recent_window {
        parts.push(format!("recent_window={value}"));
    }
    if let Some(value) = overrides.dream_depth {
        parts.push(format!("dream_depth={value}"));
    }
    if let Some(value) = overrides.cognition_size {
        parts.push(format!("cognition_size={value}"));
    }
    if let Some(value) = &overrides.recollection_level {
        parts.push(format!("recollection_level={value}"));
    }
    if let Some(value) = overrides.recollection_size {
        parts.push(format!("recollection_size={value}"));
    }
    if let Some(value) = overrides.experience_size {
        parts.push(format!("experience_size={value}"));
    }
    parts.join("&")
}

pub(crate) struct StorageClient<'a> {
    client: &'a Client,
}

impl<'a> StorageClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn upload(
        &self,
        request: &UploadStorage,
    ) -> Result<StorageResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "storage")
    }

    pub(crate) async fn show(&self, request: &GetStorage) -> Result<StorageResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "storage")
    }

    pub(crate) async fn list(&self, request: &ListStorage) -> Result<StorageResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "storage")
    }

    pub(crate) async fn remove(
        &self,
        request: &RemoveStorage,
    ) -> Result<StorageResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "storage")
    }
}

pub(crate) struct MemoryClient<'a> {
    client: &'a Client,
}

impl<'a> MemoryClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn add(&self, request: &AddMemory) -> Result<MemoryResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "memory")
    }

    pub(crate) async fn list(&self, request: &ListMemories) -> Result<MemoryResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "memory")
    }

    pub(crate) async fn get(&self, request: &GetMemory) -> Result<MemoryResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "memory")
    }
}

pub(crate) struct ExperienceClient<'a> {
    client: &'a Client,
}

impl<'a> ExperienceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "experience")
    }

    pub(crate) async fn list(
        &self,
        request: &ListExperiences,
    ) -> Result<ExperienceResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "experience")
    }

    pub(crate) async fn get(
        &self,
        request: &GetExperience,
    ) -> Result<ExperienceResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "experience")
    }
}

pub(crate) struct ConnectionClient<'a> {
    client: &'a Client,
}

impl<'a> ConnectionClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "connection")
    }

    pub(crate) async fn list(
        &self,
        request: &ListConnections,
    ) -> Result<ConnectionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "connection")
    }

    pub(crate) async fn get(
        &self,
        request: &GetConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "connection")
    }

    pub(crate) async fn remove(
        &self,
        request: &RemoveConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "connection")
    }
}

pub(crate) struct PressureClient<'a> {
    client: &'a Client,
}

impl<'a> PressureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(&self, request: &GetPressure) -> Result<PressureResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "pressure")
    }
}

pub(crate) struct PersonaClient<'a> {
    client: &'a Client,
}

impl<'a> PersonaClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, request: &SetPersona) -> Result<PersonaResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "persona")
    }

    pub(crate) async fn get(&self, request: &GetPersona) -> Result<PersonaResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "persona")
    }

    pub(crate) async fn list(
        &self,
        request: &ListPersonas,
    ) -> Result<PersonaResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "persona")
    }
}

pub(crate) struct LevelClient<'a> {
    client: &'a Client,
}

impl<'a> LevelClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, request: &SetLevel) -> Result<LevelResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "level")
    }

    pub(crate) async fn get(&self, request: &GetLevel) -> Result<LevelResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "level")
    }

    pub(crate) async fn list(&self, request: &ListLevels) -> Result<LevelResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "level")
    }
}

pub(crate) struct TextureClient<'a> {
    client: &'a Client,
}

impl<'a> TextureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(&self, request: &GetTexture) -> Result<TextureResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "texture")
    }
}

pub(crate) struct SensationClient<'a> {
    client: &'a Client,
}

impl<'a> SensationClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(
        &self,
        request: &GetSensation,
    ) -> Result<SensationResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "sensation")
    }
}

pub(crate) struct NatureClient<'a> {
    client: &'a Client,
}

impl<'a> NatureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(&self, request: &GetNature) -> Result<NatureResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "nature")
    }
}

pub(crate) struct UrgeClient<'a> {
    client: &'a Client,
}

impl<'a> UrgeClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(&self, request: &GetUrge) -> Result<UrgeResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "urge")
    }
}

pub(crate) struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn search(
        &self,
        request: &SearchQuery,
    ) -> Result<SearchResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "search")
    }
}

pub(crate) struct TrailClient<'a> {
    client: &'a Client,
}

impl<'a> TrailClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn of(&self, request: &TrailOf) -> Result<TrailResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "trail")
    }

    pub(crate) async fn from(&self, request: &TrailFrom) -> Result<TrailResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "trail")
    }
}

pub(crate) struct TenantClient<'a> {
    client: &'a Client,
}

impl<'a> TenantClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateTenant,
    ) -> Result<TenantResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "tenant")
    }

    pub(crate) async fn list(&self, request: &ListTenants) -> Result<TenantResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "tenant")
    }
}

pub(crate) struct ActorClient<'a> {
    client: &'a Client,
}

impl<'a> ActorClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(&self, request: &CreateActor) -> Result<ActorResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "actor")
    }

    pub(crate) async fn get(&self, request: &GetActor) -> Result<ActorResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "actor")
    }

    pub(crate) async fn list(&self, request: &ListActors) -> Result<ActorResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "actor")
    }
}

pub(crate) struct ProjectClient<'a> {
    client: &'a Client,
}

impl<'a> ProjectClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateProject,
    ) -> Result<ProjectResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "project")
    }

    pub(crate) async fn get(&self, request: &GetProject) -> Result<ProjectResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "project")
    }

    pub(crate) async fn list(
        &self,
        request: &ListProjects,
    ) -> Result<ProjectResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "project")
    }
}

pub(crate) struct TicketClient<'a> {
    client: &'a Client,
}

impl<'a> TicketClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn issue(
        &self,
        request: &CreateTicket,
    ) -> Result<TicketResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "ticket")
    }

    pub(crate) async fn get(&self, request: &GetTicket) -> Result<TicketResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "ticket")
    }

    pub(crate) async fn list(&self, request: &ListTickets) -> Result<TicketResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "ticket")
    }

    pub(crate) async fn validate(
        &self,
        request: &ValidateTicket,
    ) -> Result<TicketResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "ticket")
    }
}

pub(crate) struct BookmarkClient<'a> {
    client: &'a Client,
}

impl<'a> BookmarkClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn list(
        &self,
        request: &ListBookmarks,
    ) -> Result<BookmarkResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "bookmark")
    }
}

pub(crate) struct HostClient<'a> {
    client: &'a Client,
}

impl<'a> HostClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn init(&self, request: &InitHost) -> Result<HostResponse, ClientError> {
        let bytes = request.execute_request(self.client).await?;
        decode(bytes, "host")
    }
}

/// Poll the server's `/health` endpoint until it responds, then wait
/// one extra tick so that `HostKey::ensure()` has completed inside
/// the spawned server task. Without this, tests that create clients
/// with `Client::from_config` race against host key generation and
/// fall back to anonymous auth — which the new middleware rejects.
async fn wait_for_server(config: &Config) -> Result<(), Box<dyn core::error::Error>> {
    let health_url = format!("{}/health", config.base_url());
    let client = reqwest::Client::new();
    for _ in 0..100 {
        if client.get(&health_url).send().await.is_ok() {
            // Server is responding — but HostKey::ensure() may not have
            // written the key file to disk yet (it runs inside the spawned
            // task just before the listener starts accepting). Give the
            // filesystem one more tick.
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err("server did not become healthy within timeout".into())
}
