use axum::Router;
use axum::routing::get;
use std::sync::Arc;

use crate::*;

pub fn router(state: Arc<ServiceState>) -> Router {
    Router::new()
        .route("/activity", get(handlers::activity::handler))
        .nest("/agents", handlers::agent::router())
        .nest("/brains", handlers::brain::router())
        .nest("/cognitions", handlers::cognition::router())
        .nest("/connections", handlers::connection::router())
        .nest("/natures", handlers::nature::router())
        .nest("/dream", handlers::dream::router())
        .nest("/sensations", handlers::sensation::router())
        .nest("/events", handlers::event::router())
        .nest("/experiences", handlers::experience::router())
        .nest("/health", handlers::health::router())
        .nest("/introspect", handlers::introspect::router())
        .nest("/lifecycle", handlers::lifecycle::router())
        .nest("/levels", handlers::level::router())
        .nest("/memories", handlers::memory::router())
        .nest("/personas", handlers::persona::router())
        .nest("/reflect", handlers::reflect::router())
        .nest("/search", handlers::search::router())
        .nest("/sense", handlers::sense::router())
        .nest("/storage", handlers::storage::router())
        .nest("/textures", handlers::texture::router())
        .with_state(state)
}
