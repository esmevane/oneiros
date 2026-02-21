use axum::Json;
use oneiros_model::NatureRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<NatureRecord>>, Error> {
    let natures = ticket.db.list_natures()?;

    Ok(Json(natures))
}
