use axum::Json;
use oneiros_model::{ContentHash, Description, StorageEntry, StorageKey};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<StorageEntry>>, Error> {
    let entries = ticket
        .db
        .list_storage()?
        .into_iter()
        .map(|(key, desc, hash)| StorageEntry {
            key: StorageKey::new(key),
            description: Description::new(desc),
            hash: ContentHash::new(hash),
        })
        .collect();

    Ok(Json(entries))
}
