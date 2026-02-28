use axum::Json;
use serde_json::Value;

use crate::*;

#[derive(serde::Deserialize)]
pub(crate) struct ImportEvent {
    pub timestamp: String,
    pub data: Value,
}

#[derive(serde::Serialize)]
pub(crate) struct ImportResponse {
    pub imported: usize,
    pub replayed: usize,
}

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
