//! HTTP layer collector — merges all domain HTTP routes into one router.

use axum::Router;

use crate::*;

/// Build the project-scoped HTTP router.
///
/// Each domain contributes its own routes and path prefix.
pub fn project_router(ctx: ProjectContext) -> Router {
    Router::new()
        // Vocabulary domains
        .merge(LevelRouter.routes())
        .merge(TextureRouter.routes())
        .merge(SensationRouter.routes())
        .merge(NatureRouter.routes())
        .merge(PersonaRouter.routes())
        .merge(UrgeRouter.routes())
        // Entity domains
        .merge(AgentRouter.routes())
        .merge(CognitionRouter.routes())
        .merge(MemoryRouter.routes())
        .merge(ExperienceRouter.routes())
        .merge(ConnectionRouter.routes())
        // Storage
        .merge(StorageRouter.routes())
        // Derived
        .merge(PressureRouter.routes())
        // Lifecycle — uses .merge() because routes are top-level (not nested)
        .merge(LifecycleRouter.routes())
        // Search
        .merge(SearchRouter.routes())
        // State
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
