use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_id): Path<EventId>,
) -> Result<Json<Event>, Error> {
    let event = ticket.service().get_event(&given_id)?;

    Ok(Json(event))
}
