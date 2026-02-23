use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Event>>, Error> {
    let natures = ticket.db.read_events()?;

    Ok(Json(natures))
}
