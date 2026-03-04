use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(events): Json<Vec<ImportEvent>>,
) -> Result<Json<ImportResponse>, Error> {
    let mut imported = 0;

    for event in &events {
        ticket.db.import_event(&event.timestamp, &event.data)?;
        imported += 1;
    }

    let replayed = ticket.db.replay(projections::BRAIN)?;

    Ok(Json(ImportResponse { imported, replayed }))
}
