use axum::Json;

use crate::*;

#[derive(serde::Serialize)]
pub(crate) struct ReplayResponse {
    pub replayed: usize,
}

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<ReplayResponse>, Error> {
    let count = ticket.db.replay(projections::brain::ALL)?;

    Ok(Json(ReplayResponse { replayed: count }))
}
