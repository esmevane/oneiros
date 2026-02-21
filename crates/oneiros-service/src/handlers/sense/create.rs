use axum::{Json, extract::Path};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<Identity<AgentId, Agent>>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(Key::Id(agent_name.clone())))?;

    let sensed = Events::Sense(SenseEvents::Sensed {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&sensed, &[])?;

    Ok(Json(agent))
}
