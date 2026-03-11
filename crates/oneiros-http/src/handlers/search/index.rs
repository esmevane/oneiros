use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<SearchRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(SearchRequests::Search(request))?))
}
