use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<EventResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_event(EventRequests::ReplayEvents(ReplayEventsRequest))?;

    Ok(Json(response))
}
