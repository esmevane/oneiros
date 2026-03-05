mod create;

use axum::{Router, routing::post};

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/", post(create::handler))
}
