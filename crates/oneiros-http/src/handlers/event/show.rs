use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<EventId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::GetEvent(
        GetEventRequest { id },
    ))?))
}
