use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<Record<MemoryId, Memory>>, Error> {
    let memory = ticket
        .db
        .get_memory(id.to_string())?
        .ok_or(NotFound::Memory(id))?;

    Ok(Json(memory))
}
