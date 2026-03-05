use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddMemoryRequest>,
) -> Result<(StatusCode, Json<MemoryResponses>), Error> {
    let memory = ticket.service().add_memory(request)?;

    Ok((StatusCode::CREATED, Json(memory)))
}
