use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), Error> {
    let agent = ticket.service().create_agent(request)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
