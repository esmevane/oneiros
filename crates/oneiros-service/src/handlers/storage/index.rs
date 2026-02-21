use axum::Json;
use oneiros_model::StorageEntryRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<StorageEntryRecord>>, Error> {
    let entries = ticket.db.list_storage()?;

    Ok(Json(entries))
}
