use axum::http::StatusCode;

pub(crate) async fn health() -> StatusCode {
    StatusCode::OK
}
