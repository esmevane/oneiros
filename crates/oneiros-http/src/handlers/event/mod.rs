mod import;
mod index;
mod replay;
mod show;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::get(index::handler))
        .route("/import", routing::post(import::handler))
        .route("/replay", routing::post(replay::handler))
        .route("/{id}", routing::get(show::handler))
}
