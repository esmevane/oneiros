use axum::Json;
use oneiros_model::LevelRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<LevelRecord>>, Error> {
    let levels = ticket.db.list_levels()?;

    Ok(Json(levels))
}
