use axum::{Json, extract::Path};
use oneiros_model::*;

use super::collector::DreamCollector;
use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });

    ticket.db.log_event(&begun, &[])?;

    let context = DreamCollector::new(&ticket.db).collect(&agent)?;

    let complete = Events::Dreaming(DreamingEvents::DreamComplete {
        agent: context.agent.clone(),
    });

    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
