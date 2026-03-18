//! HTTP layer collector — merges all domain HTTP routes into one router.

use axum::Router;

use crate::contexts::ProjectContext;
use crate::contexts::SystemContext;
use crate::domains;

/// Build the project-scoped HTTP router.
///
/// Each domain contributes its own routes. Domains without http.rs
/// simply don't appear here.
pub fn project_router(ctx: ProjectContext) -> Router {
    Router::new()
        // Vocabulary domains
        .nest("/levels", domains::level::features::http::routes())
        .nest("/textures", domains::texture::features::http::routes())
        .nest("/sensations", domains::sensation::features::http::routes())
        .nest("/natures", domains::nature::features::http::routes())
        .nest("/personas", domains::persona::features::http::routes())
        .nest("/urges", domains::urge::features::http::routes())
        // Entity domains
        .nest("/agents", domains::agent::features::http::routes())
        .nest("/cognitions", domains::cognition::features::http::routes())
        .nest("/memories", domains::memory::features::http::routes())
        .nest("/experiences", domains::experience::features::http::routes())
        .nest("/connections", domains::connection::features::http::routes())
        // Derived
        .nest("/pressures", domains::pressure::features::http::routes())
        // Lifecycle
        .merge(domains::lifecycle::features::http::routes())
        // Search
        .nest("/search", domains::search::features::http::routes())
        // State
        .with_state(ctx)
}

/// Build the system-scoped HTTP router.
pub fn system_router(ctx: SystemContext) -> Router {
    Router::new()
        .nest("/tenants", domains::tenant::features::http::routes())
        .nest("/actors", domains::actor::features::http::routes())
        .nest("/tickets", domains::ticket::features::http::routes())
        .nest("/brains", domains::brain::features::http::routes())
        .with_state(ctx)
}
