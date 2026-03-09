use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), Error> {
    request.name = name;

    let response = ticket
        .service()
        .dispatch_agent(AgentRequests::UpdateAgent(request))?;

    Ok((StatusCode::OK, Json(response)))
}
