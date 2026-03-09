use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<MemoryResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_memory(MemoryRequests::GetMemory(GetMemoryRequest { id }))?;

    Ok(Json(response))
}
