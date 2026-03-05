use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<LifecycleResponses>), Error> {
    let response = ticket.service().emerge(request)?;

    Ok((StatusCode::CREATED, Json(response)))
}
