use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddMemoryRequest>,
) -> Result<(StatusCode, Json<Memory>), Error> {
    let memory = ticket.service().add_memory(request)?;

    Ok((StatusCode::CREATED, Json(memory)))
}
