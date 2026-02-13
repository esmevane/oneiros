mod create;

use axum::{Router, routing};
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/{agent_name}", routing::post(create::handler))
}
