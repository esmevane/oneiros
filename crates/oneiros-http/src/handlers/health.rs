use axum::{Router, http::StatusCode, routing::get};

pub(crate) fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/", get(index))
}

async fn index() -> StatusCode {
    StatusCode::OK
}
