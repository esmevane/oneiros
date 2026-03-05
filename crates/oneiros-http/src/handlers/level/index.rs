use axum::Json;
use oneiros_model::LevelResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<LevelResponses>, Error> {
    let levels = ticket.service().list_levels()?;

    Ok(Json(levels))
}
