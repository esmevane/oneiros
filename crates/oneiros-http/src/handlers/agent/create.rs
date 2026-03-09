use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), Error> {
    let response = ticket
        .service()
        .dispatch_agent(AgentRequests::CreateAgent(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
