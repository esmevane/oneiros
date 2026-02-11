use axum::Router;
use std::sync::Arc;

use crate::*;

pub fn router(state: Arc<ServiceState>) -> Router {
    Router::new()
        .nest("/brains", handlers::brain::router())
        .nest("/health", handlers::health::router())
        .nest("/personas", handlers::persona::router())
        .with_state(state)
}
