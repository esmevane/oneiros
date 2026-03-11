use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListEventsRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::ListEvents(request))?))
}
