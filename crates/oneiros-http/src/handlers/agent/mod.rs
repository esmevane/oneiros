mod create;
mod delete;
mod index;
mod show;
mod update;

use axum::{Router, routing};

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::post(create::handler))
        .route("/", routing::get(index::handler))
        .route("/{name}", routing::get(show::handler))
        .route("/{name}", routing::put(update::handler))
        .route("/{name}", routing::delete(delete::handler))
}
