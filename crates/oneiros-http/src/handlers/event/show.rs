use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<EventId>,
) -> Result<Json<EventResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_event(EventRequests::GetEvent(GetEventRequest { id }))?;

    Ok(Json(response))
}
