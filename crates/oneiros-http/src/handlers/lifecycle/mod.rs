mod emerge;
mod recede;
mod sleep;
mod wake;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new()
        .route("/wake/{agent_name}", routing::post(wake::handler))
        .route("/sleep/{agent_name}", routing::post(sleep::handler))
        .route("/emerge", routing::post(emerge::handler))
        .route("/recede/{agent_name}", routing::post(recede::handler))
}
