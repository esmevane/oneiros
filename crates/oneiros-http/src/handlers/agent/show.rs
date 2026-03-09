use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_agent(AgentRequests::GetAgent(GetAgentRequest { name }))?;

    Ok(Json(response))
}
