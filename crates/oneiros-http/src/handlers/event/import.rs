use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(events): Json<Vec<ImportEvent>>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::ImportEvents(
        ImportEventsRequest { events },
    ))?))
}
