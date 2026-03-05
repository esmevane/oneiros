use axum::Router;
use axum::routing::get;
use std::sync::Arc;

use crate::*;

pub(crate) fn router() -> Router<Arc<ServiceState>> {
    Router::new().route("/", get(index::handler))
}

mod index {
    use axum::http::StatusCode;

    pub(crate) async fn handler() -> StatusCode {
        StatusCode::OK
    }
}
