use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_id): Path<EventId>,
) -> Result<Json<Event>, Error> {
    let nature = ticket
        .db
        .get_event(&given_id)?
        .ok_or(NotFound::Event(given_id))?;

    Ok(Json(nature))
}
