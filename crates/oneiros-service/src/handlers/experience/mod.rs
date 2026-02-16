mod add_ref;
mod create;
mod index;
mod show;
mod update_description;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::post(create::handler))
        .route("/", routing::get(index::handler))
        .route("/{id}", routing::get(show::handler))
        .route("/{id}/refs", routing::post(add_ref::handler))
        .route(
            "/{id}/description",
            routing::put(update_description::handler),
        )
}
