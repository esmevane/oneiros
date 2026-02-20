use axum::Json;
use axum::extract::Path;
use oneiros_model::{Link, Record, StorageEntry, StorageKey, StorageRef};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<StorageEntry>>, Error> {
    let by_key = StorageRef(identifier.clone())
        .decode()
        .ok()
        .and_then(|key| ticket.db.get_storage(&key).transpose())
        .transpose()?;

    let entry = if let Some(e) = by_key {
        e
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_storage_by_link(link.to_string())?
            .ok_or(NotFound::Storage(StorageKey::new(&identifier)))?
    } else {
        return Err(NotFound::Storage(StorageKey::new(&identifier)).into());
    };

    let record = Record::new(entry)?;
    Ok(Json(record))
}
