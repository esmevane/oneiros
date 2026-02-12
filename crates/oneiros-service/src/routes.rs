use axum::Router;
use std::sync::Arc;

use crate::*;

pub fn router(state: Arc<ServiceState>) -> Router {
    Router::new()
        .nest("/agents", handlers::agent::router())
        .nest("/brains", handlers::brain::router())
        .nest("/health", handlers::health::router())
        .nest("/levels", handlers::level::router())
        .nest("/personas", handlers::persona::router())
        .nest("/textures", handlers::texture::router())
        .with_state(state)
}
