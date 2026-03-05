use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<Json<LevelResponses>, Error> {
    let level = ticket.service().set_level(level)?;

    Ok(Json(level))
}
