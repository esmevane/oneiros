use std::sync::Arc;

use aide::{
    axum::{ApiRouter, routing as api_routing},
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{Json, Router, extract::State, response::Html, routing};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, tower::StreamableHttpService,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

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
pub struct Server {
    config: Config,
}

impl Server {
    /// Create a server from an engine.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Serve on a pre-bound TCP listener. Loads/generates the host secret
    /// key, binds an iroh Bridge, registers the sync protocol handler
    /// against it, then assembles the router.
    pub async fn serve(self, listener: TcpListener) -> Result<(), ServerError> {
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
    pub async fn start(self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;

        self.serve(listener).await
    }

    /// Build a router from an already-constructed state. Used by `serve`
    /// once the async bridge binding has completed.
    pub fn router_from_state(state: ServerState) -> Router {
        /// Serves the OpenAPI spec as JSON. Pulled from state — populated
        /// once after router assembly to avoid a global `.layer(Extension)`
        /// walk over every route on each server build.
        async fn serve_api(State(state): State<ServerState>) -> Json<OpenApi> {
            Json(state.api().cloned().unwrap_or_default())
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

        let mut api = OpenApi::default();
        let app_docs = AppDocs;

        let router = ApiRouter::new()
            .merge(root)
            .nest_service("/mcp", Router::new().route_service("/", mcp_service))
            .merge(ActorRouter.routes())
            .merge(AgentRouter.routes())
            .merge(BookmarkRouter.routes())
            .merge(BrainRouter.routes())
            .merge(CognitionRouter.routes())
            .merge(ConnectionRouter.routes())
            .merge(ContinuityRouter.routes())
            .merge(ExperienceRouter.routes())
            .merge(LevelRouter.routes())
            .merge(MemoryRouter.routes())
            .merge(NatureRouter.routes())
            .merge(PeerRouter.routes())
            .merge(PersonaRouter.routes())
            .merge(PressureRouter.routes())
            .merge(ProjectRouter.routes())
            .merge(SearchRouter.routes())
            .merge(SeedRouter.routes())
            .merge(SensationRouter.routes())
            .merge(StorageRouter.routes())
            .merge(SystemRouter.routes())
            .merge(TenantRouter.routes())
            .merge(TextureRouter.routes())
            .merge(TicketRouter.routes())
            .merge(UrgeRouter.routes())
            // OpenAPI spec and docs
            .route("/api.json", api_routing::get(serve_api))
            .route("/docs", Scalar::new("/api.json").axum_route())
            .finish_api_with(&mut api, |mut api| {
                api = api
                    .title(app_docs.title().as_str())
                    .version(app_docs.version().as_str())
                    .description(app_docs.description().as_str())
                    .security_scheme(
                        app_docs.security_scheme_name().as_str(),
                        aide::openapi::SecurityScheme::Http {
                            scheme: "bearer".into(),
                            bearer_format: None,
                            description: Some(app_docs.security_scheme_description().to_string()),
                            extensions: Default::default(),
                        },
                    );

                for tag in app_docs.tags() {
                    api = api.tag(aide::openapi::Tag {
                        name: tag.name.to_string(),
                        description: Some(tag.description.to_string()),
                        ..Default::default()
                    });
                }

                api
            });

        state.set_api(api);
        router.with_state(state).layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
    }
}
