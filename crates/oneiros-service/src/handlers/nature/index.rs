use axum::Json;
use oneiros_model::Nature;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Nature>>, Error> {
    let natures = ticket.db.list_natures()?;

    Ok(Json(natures))
}
