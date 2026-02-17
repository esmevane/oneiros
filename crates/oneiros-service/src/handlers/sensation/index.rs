use axum::Json;
use oneiros_model::Sensation;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Sensation>>, Error> {
    let sensations = ticket.db.list_sensations()?;

    Ok(Json(sensations))
}
