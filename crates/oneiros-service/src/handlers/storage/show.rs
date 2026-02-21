use axum::Json;
use axum::extract::Path;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<StorageKey, StorageLink>>,
) -> Result<Json<Record<StorageEntry>>, Error> {
    // Storage keys may arrive as StorageRef-encoded (base32). Try decoding.
    let key = match &key {
        Key::Id(storage_key) => match StorageRef(storage_key.as_str().to_owned()).decode() {
            Ok(decoded) => Key::Id(decoded),
            Err(_) => key,
        },
        _ => key,
    };

    let entry = ticket
        .db
        .get_storage_by_key(&key)?
        .ok_or(NotFound::Storage(key))?;

    let record = Record::new(entry)?;
    Ok(Json(record))
}
