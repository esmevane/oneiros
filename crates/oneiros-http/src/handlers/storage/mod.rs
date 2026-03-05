mod content;
mod delete;
mod index;
mod show;
mod update;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::get(index::handler))
        .route("/{key}", routing::put(update::handler))
        .route("/{key}", routing::get(show::handler))
        .route("/{key}", routing::delete(delete::handler))
        .route("/{key}/content", routing::get(content::handler))
}
