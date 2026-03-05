mod create;
mod index;
mod show;
mod update_description;
mod update_sensation;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/", routing::post(create::handler))
        .route("/", routing::get(index::handler))
        .route("/{id}", routing::get(show::handler))
        .route(
            "/{id}/description",
            routing::put(update_description::handler),
        )
        .route("/{id}/sensation", routing::put(update_sensation::handler))
}
