use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<AgentRecord>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let sensed = Events::Sense(SenseEvents::Sensed {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&sensed, &[])?;

    Ok(Json(agent))
}
