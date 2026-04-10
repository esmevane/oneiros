//! Test harness — a first-class testing surface for the engine.
//!
//! `TestApp` owns the full lifecycle: config, server, and teardown.
//! Tests interact through the same layers the real system uses:
//! CLI commands go through clap → client → HTTP → service → event log.
//!
//! ```rust,ignore
//! let app = TestApp::new()
//!     .await?
//!     .init_system()
//!     .await?
//!     .init_project()
//!     .await?
//!     .seed_core()
//!     .await?;
//!
//! app.command("agent create thinker process").await?;
//!
//! let client = app.client();
//! let persona = client.persona().get(&PersonaName::new("process")).await?;
//! ```

use clap::Parser;

use crate::*;

/// A running test application with an isolated data directory and server.
///
/// Every `TestApp` gets its own tempdir and ephemeral-port server.
/// Commands and client calls go through the full HTTP stack.
pub struct TestApp {
    engine: Engine,
    _dir: tempfile::TempDir,
    _handle: ServerHandle,
}

impl TestApp {
    /// Boot a new test app: tempdir, config, server on a random port.
    pub async fn new() -> Result<Self, Error> {
        let dir = tempfile::tempdir().expect("create tempdir");

        let config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .brain(BrainName::new("test"))
            .output(OutputMode::Json)
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse().unwrap())
                    .build(),
            )
            .build();

        let mut engine = Engine::new(config);
        let handle = engine
            .start()
            .await
            .map_err(|e| Error::Context(e.to_string()))?;

        Ok(Self {
            engine,
            _dir: dir,
            _handle: handle,
        })
    }

    /// Initialize the system database and default tenant/actor.
    pub async fn init_system(self) -> Result<Self, Error> {
        self.command("system init --name test").await?;
        Ok(self)
    }

    /// Initialize a project (brain) with the default name.
    pub async fn init_project(self) -> Result<Self, Error> {
        let brain = self.engine.config().brain.to_string();
        self.command(&format!("project init --name {brain}"))
            .await?;
        Ok(self)
    }

    /// Seed the core vocabulary (textures, levels, sensations, etc.).
    pub async fn seed_core(self) -> Result<Self, Error> {
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
    pub async fn command(&self, input: &str) -> Result<Rendered<Responses>, Error> {
        // Build the arg list: "oneiros" + the user's command tokens.
        // Prepend global flags so clap routes to the right config.
        let config = self.engine.config();
        let mut args: Vec<String> = vec![
            "oneiros".into(),
            "--data-dir".into(),
            config.data_dir.display().to_string(),
            "--brain".into(),
            config.brain.to_string(),
        ];
        args.extend(
            shlex::split(input)
                .unwrap_or_else(|| panic!("invalid shell words in command: {input}")),
        );

        let cli = Cli::try_parse_from(&args).map_err(|e| Error::Context(e.to_string()))?;

        // Execute against our engine's config (which has the correct port).
        self.engine.execute(&cli).await
    }

    /// Get a typed HTTP client connected to this app's server.
    ///
    /// The client authenticates with the project's token (written during
    /// `init_project`). Domain-specific clients are available via methods
    /// like `.persona()`, `.agent()`, etc.
    pub fn client(&self) -> TestClient {
        let config = self.engine.config();
        let client = match config.token() {
            Some(token) => Client::with_token(config.base_url(), token),
            None => Client::new(config.base_url()),
        };

        TestClient { client }
    }

    /// The base URL of the running server (e.g. `http://127.0.0.1:PORT`).
    pub fn base_url(&self) -> String {
        self.engine.config().base_url()
    }

    /// The MCP endpoint URL (e.g. `http://127.0.0.1:PORT/mcp`).
    pub fn mcp_url(&self) -> String {
        format!("{}/mcp", self.base_url())
    }

    /// The project token, if one exists (written during `init_project`).
    pub fn token(&self) -> Option<Token> {
        self.engine.config().token()
    }

    /// Shared engine config. Useful for tests that need to construct a
    /// `SystemContext` or `ProjectContext` directly against the running
    /// instance (rather than going through an HTTP client).
    pub fn config(&self) -> &Config {
        self.engine.config()
    }
}

/// A typed HTTP client for test assertions.
///
/// Wraps the existing domain clients behind a fluent API.
/// Each method returns the production domain client — no test doubles.
pub struct TestClient {
    client: Client,
}

impl TestClient {
    pub fn persona(&self) -> PersonaClient<'_> {
        PersonaClient::new(&self.client)
    }

    pub fn agent(&self) -> AgentClient<'_> {
        AgentClient::new(&self.client)
    }

    pub fn level(&self) -> LevelClient<'_> {
        LevelClient::new(&self.client)
    }

    pub fn texture(&self) -> TextureClient<'_> {
        TextureClient::new(&self.client)
    }

    pub fn sensation(&self) -> SensationClient<'_> {
        SensationClient::new(&self.client)
    }

    pub fn nature(&self) -> NatureClient<'_> {
        NatureClient::new(&self.client)
    }

    pub fn urge(&self) -> UrgeClient<'_> {
        UrgeClient::new(&self.client)
    }

    pub fn cognition(&self) -> CognitionClient<'_> {
        CognitionClient::new(&self.client)
    }

    pub fn memory(&self) -> MemoryClient<'_> {
        MemoryClient::new(&self.client)
    }

    pub fn experience(&self) -> ExperienceClient<'_> {
        ExperienceClient::new(&self.client)
    }

    pub fn connection(&self) -> ConnectionClient<'_> {
        ConnectionClient::new(&self.client)
    }

    pub fn storage(&self) -> StorageClient<'_> {
        StorageClient::new(&self.client)
    }

    pub fn continuity(&self) -> ContinuityClient<'_> {
        ContinuityClient::new(&self.client)
    }

    pub fn search(&self) -> SearchClient<'_> {
        SearchClient::new(&self.client)
    }

    pub fn pressure(&self) -> PressureClient<'_> {
        PressureClient::new(&self.client)
    }

    pub fn tenant(&self) -> TenantClient<'_> {
        TenantClient::new(&self.client)
    }

    pub fn actor(&self) -> ActorClient<'_> {
        ActorClient::new(&self.client)
    }

    pub fn brain(&self) -> BrainClient<'_> {
        BrainClient::new(&self.client)
    }

    pub fn ticket(&self) -> TicketClient<'_> {
        TicketClient::new(&self.client)
    }

    pub fn bookmark(&self) -> BookmarkClient<'_> {
        BookmarkClient::new(&self.client)
    }

    pub fn peer(&self) -> PeerClient<'_> {
        PeerClient::new(&self.client)
    }
}
