use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Agent>), Error> {
    let agent = ticket.service().emerge(request)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
