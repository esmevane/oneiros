mod delete;
mod index;
mod show;
mod update;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::put(update::handler))
        .route("/", routing::get(index::handler))
        .route("/{name}", routing::get(show::handler))
        .route("/{name}", routing::delete(delete::handler))
}
