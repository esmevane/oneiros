use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListMemoriesRequest>,
) -> Result<Json<MemoryResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_memory(MemoryRequests::ListMemories(request))?;

    Ok(Json(response))
}
