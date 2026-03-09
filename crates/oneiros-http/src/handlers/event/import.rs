use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(events): Json<Vec<ImportEvent>>,
) -> Result<Json<EventResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_event(EventRequests::ImportEvents(ImportEventsRequest { events }))?;

    Ok(Json(response))
}
