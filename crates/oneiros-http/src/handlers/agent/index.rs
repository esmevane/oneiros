use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<AgentResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_agent(AgentRequests::ListAgents(ListAgentsRequest))?;

    Ok(Json(response))
}
