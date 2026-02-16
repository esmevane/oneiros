mod index;
mod remove;
mod set;
mod show;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::put(set::handler))
        .route("/", routing::get(index::handler))
        .route("/{name}", routing::get(show::handler))
        .route("/{name}", routing::delete(remove::handler))
}
