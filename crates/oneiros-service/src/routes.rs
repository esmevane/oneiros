use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::handlers;
use crate::state::ServiceState;

pub fn router(state: Arc<ServiceState>) -> Router {
    Router::new()
        .route("/brains", post(handlers::brain::create_brain))
        .route("/health", get(handlers::health::health))
        .with_state(state)
}
