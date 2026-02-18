use axum::{Json, extract::Path};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<Agent>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let begun = Events::Reflecting(ReflectingEvents::ReflectionBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let complete = Events::Reflecting(ReflectingEvents::ReflectionComplete {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&complete, &[])?;

    Ok(Json(agent))
}
