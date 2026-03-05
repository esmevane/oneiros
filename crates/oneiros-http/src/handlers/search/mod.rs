mod index;

use axum::{Router, routing};

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/", routing::get(index::handler))
}
