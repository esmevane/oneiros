mod create;

use axum::{Router, routing::post};

pub(crate) use create::CreateBrainError;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/", post(create::handler))
}
