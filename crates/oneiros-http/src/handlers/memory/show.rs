use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<MemoryResponses>, Error> {
    let memory = ticket.service().get_memory(&id)?;

    Ok(Json(memory))
}
