use axum::{Json, Router, extract::Query, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new().route("/", routing::get(index))
}

async fn index(
    ticket: OneirosContext,
    Query(request): Query<SearchRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(SearchRequests::Search(request))?))
}
