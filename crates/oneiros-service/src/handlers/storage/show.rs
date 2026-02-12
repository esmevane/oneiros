use axum::Json;
use axum::extract::Path;
use oneiros_model::{ContentHash, Description, StorageEntry, StorageKey, StorageRef};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<StorageEntry>, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let (k, desc, hash) = ticket.db.get_storage(&key)?.ok_or(NotFound::Storage(key))?;

    Ok(Json(StorageEntry {
        key: StorageKey::new(k),
        description: Description::new(desc),
        hash: ContentHash::new(hash),
    }))
}
