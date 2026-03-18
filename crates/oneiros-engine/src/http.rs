//! HTTP layer collector — merges all domain HTTP routes into one router.

use axum::Router;

use crate::contexts::ProjectContext;
use crate::contexts::SystemContext;
use crate::domains;

/// Build the project-scoped HTTP router.
///
/// Each domain contributes its own routes and path prefix.
/// Domains without http.rs simply don't appear here.
pub fn project_router(ctx: ProjectContext) -> Router {
    use domains::*;

    Router::new()
        // Vocabulary domains
        .nest(level::features::http::PATH, level::features::http::routes())
        .nest(
            texture::features::http::PATH,
            texture::features::http::routes(),
        )
        .nest(
            sensation::features::http::PATH,
            sensation::features::http::routes(),
        )
        .nest(
            nature::features::http::PATH,
            nature::features::http::routes(),
        )
        .nest(
            persona::features::http::PATH,
            persona::features::http::routes(),
        )
        .nest(urge::features::http::PATH, urge::features::http::routes())
        // Entity domains
        .nest(agent::features::http::PATH, agent::features::http::routes())
        .nest(
            cognition::features::http::PATH,
            cognition::features::http::routes(),
        )
        .nest(
            memory::features::http::PATH,
            memory::features::http::routes(),
        )
        .nest(
            experience::features::http::PATH,
            experience::features::http::routes(),
        )
        .nest(
            connection::features::http::PATH,
            connection::features::http::routes(),
        )
        // Storage
        .nest(
            storage::features::http::PATH,
            storage::features::http::routes(),
        )
        // Derived
        .nest(
            pressure::features::http::PATH,
            pressure::features::http::routes(),
        )
        // Lifecycle
        .merge(lifecycle::features::http::routes())
        // Search
        .nest(
            search::features::http::PATH,
            search::features::http::routes(),
        )
        // State
        .with_state(ctx)
}

/// Build the system-scoped HTTP router.
pub fn system_router(ctx: SystemContext) -> Router {
    use domains::*;

    Router::new()
        .nest(
            tenant::features::http::PATH,
            tenant::features::http::routes(),
        )
        .nest("/", ActorRouter.routes())
        .nest(
            ticket::features::http::PATH,
            ticket::features::http::routes(),
        )
        .nest(brain::features::http::PATH, brain::features::http::routes())
        .with_state(ctx)
}
