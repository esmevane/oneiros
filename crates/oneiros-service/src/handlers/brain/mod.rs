mod create;

use axum::{Router, routing};

pub(crate) use create::CreateBrainError;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/", routing::post(create::handler))
}
