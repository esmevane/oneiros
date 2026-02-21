use axum::Json;
use oneiros_model::SensationRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<SensationRecord>>, Error> {
    let sensations = ticket.db.list_sensations()?;

    Ok(Json(sensations))
}
