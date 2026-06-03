use std::{net::SocketAddr, sync::Arc};

use aide::{
    axum::{ApiRouter, routing as api_routing},
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{Json, Router, extract::State, middleware, routing};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, tower::StreamableHttpService,
};
use schemars::JsonSchema;
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ServerError {
    #[error(transparent)]
    Listener(#[from] std::io::Error),
    #[error(transparent)]
    State(#[from] ServerStateError),
    #[error(transparent)]
    Migration(#[from] MigrationError),
}

/// Response body for the health-check endpoint.
#[derive(Debug, Serialize, JsonSchema)]
struct HealthResponse {
    status: String,
    version: String,
}

/// GET `/health` — server liveness check.
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

/// An HTTP server backed by a `ServerState`.
pub(crate) struct Server {
    config: Config,
}

impl Server {
    /// Create a server from an engine.
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// Bind the configured address and serve until the server stops.
    /// The caller's task becomes the server.
    pub(crate) async fn serve(self) -> Result<(), ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;
        self.serve_on(listener).await
    }

    /// Bind the configured address and spawn the server into a background
    /// task. Returns a handle carrying the resolved address (useful when
    /// the configured port is `0`) and the task. The server stops when the
    /// handle is dropped.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "We're using this in tests only, now - but might expand later to provide embedded paths"
        )
    )]
    pub(crate) async fn spawn(self) -> Result<ServerHandle, ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;
        let address = listener.local_addr()?;

        let handle = tokio::spawn(async move {
            if let Err(err) = self.serve_on(listener).await {
                eprintln!("server exited with error: {err}");
            }
        });

        Ok(ServerHandle { address, handle })
    }

    /// Serve on a pre-bound TCP listener. Loads/generates the host secret
    /// key, binds an iroh Bridge, registers the sync protocol handler
    /// against it, then assembles the router. Shared inner used by both
    /// `serve` and `spawn`.
    async fn serve_on(self, listener: TcpListener) -> Result<(), ServerError> {
        match MigrationService::ensure_current(&self.config)? {
            MigrationOutcome::AlreadyCurrent => {}
            MigrationOutcome::Migrated {
                applied,
                backup_path,
            } => {
                tracing::info!(
                    applied = ?applied,
                    backup = %backup_path.display(),
                    "data-dir migrated forward to current layout",
                );
            }
        }
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

    /// Build a router from an already-constructed state. Used by `serve`
    /// once the async bridge binding has completed.
    pub(crate) fn router_from_state(state: ServerState) -> Router {
        /// Serves the OpenAPI spec as JSON. Pulled from state — populated
        /// once after router assembly to avoid a global `.layer(Extension)`
        /// walk over every route on each server build.
        async fn serve_api(State(state): State<ServerState>) -> Json<OpenApi> {
            Json(state.api().cloned().unwrap_or_default())
        }

        let root = Router::new()
            .route(
                "/",
                routing::get(|| async { DashboardAssets::index_html() }),
            )
            .route("/dashboard/config", routing::get(dashboard_config));

        // Serve static dashboard assets (JS, CSS, favicons) from the
        // embedded SPA. Any path under /_astro/ or named asset files
        // are resolved against the compiled-in dashboard dist.
        let assets = Router::new().route(
            "/{*path}",
            routing::get(
                |axum::extract::Path(path): axum::extract::Path<String>| async move {
                    DashboardAssets::serve(&path)
                },
            ),
        );

        state.hydrate();

        let mut api = OpenApi::default();
        let app_docs = AppDocs;

        let router = ApiRouter::new()
            .merge(root)
            .merge(assets)
            .nest_service(
                "/mcp",
                Router::new().route_service("/", {
                    let mcp_state = state.clone();
                    StreamableHttpService::new(
                        move || Ok(EngineToolBox::new(mcp_state.clone())),
                        Arc::new(LocalSessionManager::default()),
                        Default::default(),
                    )
                }),
            )
            .merge(ActorRouter.routes())
            .merge(AgentRouter.routes())
            .merge(BookmarkRouter.routes())
            .merge(CognitionRouter.routes())
            .merge(ConnectionRouter.routes())
            .merge(ContinuityRouter.routes())
            .merge(ExperienceRouter.routes())
            .merge(FollowRouter.routes())
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
            .merge(HostRouter.routes())
            .merge(LensRouter.routes())
            .merge(SliceRouter.routes())
            .merge(TenantRouter.routes())
            .merge(TextureRouter.routes())
            .merge(TicketRouter.routes())
            .merge(TrailRouter.routes())
            .merge(UrgeRouter.routes())
            // Health check
            .api_route(
                "/health",
                api_routing::get_with(health_check, |op| {
                    op.tag("health")
                        .summary("Server health")
                        .description("Returns the server status and engine version.")
                        .response::<200, Json<HealthResponse>>()
                }),
            )
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

        router
            .with_state(state.clone())
            .layer(middleware::from_fn_with_state(state, auth_middleware))
            .layer(
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

/// A handle to a server running in a background task.
///
/// Holds the resolved address (the actual bound port — useful when the
/// configured address used port `0`) and the task. The server is aborted
/// when the handle is dropped.
pub(crate) struct ServerHandle {
    address: SocketAddr,
    handle: tokio::task::JoinHandle<()>,
}

impl ServerHandle {
    /// The address the server is actually listening on.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "We're using this in tests only, now - but might expand later to provide embedded paths"
        )
    )]
    pub(crate) fn address(&self) -> SocketAddr {
        self.address
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn spawn_binds_ephemeral_port() {
        let dir = tempfile::TempDir::new().unwrap();
        let config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .project(ProjectName::new("test"))
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse().unwrap())
                    .build(),
            )
            .build();

        let handle = Server::new(config).spawn().await.unwrap();

        assert_ne!(handle.address().port(), 0, "should resolve to a real port");
    }
}
