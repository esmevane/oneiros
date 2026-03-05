mod add;
mod index;
mod show;

use axum::{Router, routing};

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::post(add::handler))
        .route("/", routing::get(index::handler))
        .route("/{id}", routing::get(show::handler))
}
