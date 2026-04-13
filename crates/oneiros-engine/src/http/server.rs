use std::sync::Arc;

use axum::{Json, Router, extract::State, response::Html, routing};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, tower::StreamableHttpService,
};
use tokio::net::TcpListener;

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Listener(#[from] std::io::Error),
    #[error(transparent)]
    State(#[from] ServerStateError),
}

/// The dashboard HTML, embedded at compile time.
const DASHBOARD_HTML: &str = include_str!("../../templates/dashboard/index.html");

/// An HTTP server backed by a `ServerState`.
pub(crate) struct Server {
    config: Config,
}

impl Server {
    /// Create a server from an engine.
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// Serve on a pre-bound TCP listener. Loads/generates the host secret
    /// key, binds an iroh Bridge, registers the sync protocol handler
    /// against it, then assembles the router.
    pub(crate) async fn serve(self, listener: TcpListener) -> Result<(), ServerError> {
        let state = ServerState::bind(self.config.clone()).await?;

        // Register the sync handler on the bridge so incoming
        // `/oneiros/sync/1` connections from peers can serve canon updates.
        state
            .bridge()
            .serve(self.config.clone(), state.canons().clone());

        let app = Self::router_from_state(state);

        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }

    /// Start the server
    pub(crate) async fn start(self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;

        self.serve(listener).await
    }

    /// Build a router from an already-constructed state. Used by `serve`
    /// once the async bridge binding has completed.
    pub(crate) fn router_from_state(state: ServerState) -> Router {
        /// Returns the dashboard bootstrap config: token + brain name.
        async fn dashboard_config(State(state): State<ServerState>) -> Json<serde_json::Value> {
            let token = state.token().map(|t| t.to_string());
            let brain = state.brain_name().to_string();
            Json(serde_json::json!({ "token": token, "brain": brain }))
        }

        let root = Router::new()
            .route("/", routing::get(async || Html(DASHBOARD_HTML)))
            .route("/health", routing::get(async || "ok"))
            .route("/dashboard/config", routing::get(dashboard_config));

        // MCP streamable HTTP transport — each session gets its own EngineToolBox
        // backed by the shared ServerState for full access to canons, config,
        // and per-request context resolution.
        state.hydrate();
        let mcp_state = state.clone();
        let mcp_service = StreamableHttpService::new(
            move || Ok(EngineToolBox::new(mcp_state.clone())),
            Arc::new(LocalSessionManager::default()),
            Default::default(),
        );

        Router::new()
            .merge(root)
            .nest("/mcp", Router::new().route_service("/", mcp_service))
            .merge(LevelRouter.routes())
            .merge(TextureRouter.routes())
            .merge(SensationRouter.routes())
            .merge(NatureRouter.routes())
            .merge(PersonaRouter.routes())
            .merge(UrgeRouter.routes())
            .merge(AgentRouter.routes())
            .merge(CognitionRouter.routes())
            .merge(MemoryRouter.routes())
            .merge(ExperienceRouter.routes())
            .merge(ConnectionRouter.routes())
            .merge(StorageRouter.routes())
            .merge(PressureRouter.routes())
            .merge(ContinuityRouter.routes())
            .merge(SearchRouter.routes())
            .merge(ProjectRouter.routes())
            .merge(TenantRouter.routes())
            .merge(ActorRouter.routes())
            .merge(TicketRouter.routes())
            .merge(BrainRouter.routes())
            .merge(BookmarkRouter.routes())
            .merge(PeerRouter.routes())
            .with_state(state)
    }
}
