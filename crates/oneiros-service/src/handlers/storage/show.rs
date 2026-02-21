use axum::Json;
use axum::extract::Path;
use oneiros_model::{StorageEntryRecord, StorageRef};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<StorageEntryRecord>, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let entry = ticket.db.get_storage(&key)?.ok_or(NotFound::Storage(key))?;

    Ok(Json(entry))
}
