use axum::Json;
use axum::extract::Path;
use oneiros_model::{StorageEntry, StorageRef};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<StorageEntry>, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let entry = ticket.service().get_storage(&key)?;

    Ok(Json(entry))
}
