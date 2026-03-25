//! HTTP layer collector — merges all domain HTTP routes into one router.

use axum::{Router, response::Html, routing};

use crate::*;

/// The dashboard HTML, embedded at compile time.
const DASHBOARD_HTML: &str = include_str!("../templates/dashboard/index.html");

/// Serve the dashboard HTML.
async fn index() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

/// Simple health check endpoint.
async fn health() -> &'static str {
    "ok"
}

/// Build the project-scoped HTTP router.
///
/// Each domain contributes its own routes and path prefix.
pub fn project_router(ctx: ProjectContext) -> Router {
    Router::new()
        .route("/", routing::get(index))
        .route("/health", routing::get(health))
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
        .with_state(ctx)
}

/// Build the system-scoped HTTP router.
pub fn system_router(ctx: SystemContext) -> Router {
    Router::new()
        .merge(TenantRouter.routes())
        .merge(ActorRouter.routes())
        .merge(TicketRouter.routes())
        .merge(BrainRouter.routes())
        .with_state(ctx)
}
