mod create;
mod index;
mod remove;
mod show;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::post(create::handler))
        .route("/", routing::get(index::handler))
        .route("/{id}", routing::get(show::handler))
        .route("/{id}", routing::delete(remove::handler))
}
