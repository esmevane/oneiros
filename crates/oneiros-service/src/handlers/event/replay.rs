use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<ReplayResponse>, Error> {
    let count = ticket.db.replay(projections::BRAIN)?;

    Ok(Json(ReplayResponse { replayed: count }))
}
