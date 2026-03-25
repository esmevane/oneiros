use axum::{Router, response::Html, routing};
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
        async fn index() -> Html<&'static str> {
            Html(DASHBOARD_HTML)
        }

        async fn health() -> &'static str {
            "ok"
        }

        let root = Router::new()
            .route("/", routing::get(index))
            .route("/health", routing::get(health));

        Router::new()
            .merge(root)
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
            .with_state(ServerState::new(self.config.clone()))
    }
}
