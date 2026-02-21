use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<MemoryId, MemoryLink>>,
) -> Result<Json<Record<Identity<MemoryId, Memory>>>, Error> {
    let memory = ticket
        .db
        .get_memory_by_key(&key)?
        .ok_or(NotFound::Memory(key))?;

    let record = Record::new(memory)?;
    Ok(Json(record))
}
