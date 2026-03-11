use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    request.name = name;

    Ok((
        StatusCode::OK,
        Json(ticket.dispatch(AgentRequests::UpdateAgent(request))?),
    ))
}
