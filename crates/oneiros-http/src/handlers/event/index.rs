use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListEventsRequest>,
) -> Result<Json<EventResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_event(EventRequests::ListEvents(request))?;

    Ok(Json(response))
}
