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
}

/// The dashboard HTML, embedded at compile time.
const DASHBOARD_HTML: &str = include_str!("../../templates/dashboard/index.html");

/// An HTTP server backed by a `ServerState`.
pub struct Server {
    config: Config,
}

impl Server {
    /// Create a server from an engine.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Serve on a pre-bound TCP listener.
    pub async fn serve(self, listener: TcpListener) -> Result<(), ServerError> {
        let app = self.router();

        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }

    /// Start the server
    pub async fn start(self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;

        self.serve(listener).await
    }

    pub fn router(&self) -> Router {
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
        // that shares the server's broadcast channel for event observability.
        let state = ServerState::new(self.config.clone());
        let mcp_state = state.clone();
        let mcp_service = StreamableHttpService::new(
            move || {
                let context = mcp_state.project_context(mcp_state.config().clone());
                Ok(EngineToolBox::new(context))
            },
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
            .with_state(state)
    }
}
