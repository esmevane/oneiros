use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<AgentName>,
    Json(request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), Error> {
    let agent = ticket.service().update_agent(&given_name, request)?;

    Ok((StatusCode::OK, Json(agent)))
}
