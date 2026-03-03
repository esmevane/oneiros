use axum::Json;
use oneiros_model::StorageEntry;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<StorageEntry>>, Error> {
    let entries = ticket.service().list_storage()?;

    Ok(Json(entries))
}
