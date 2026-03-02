use axum::{
    Json,
    extract::{Path, Query},
};
use oneiros_model::*;

use super::collector::{DreamCollector, DreamParams};
use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
    Query(params): Query<DreamParams>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });

    ticket.db.log_event(&begun, &[])?;
    ticket.broadcast(&begun);

    let context = DreamCollector::new(&ticket.db, params.into()).collect(&agent)?;

    let complete = Events::Dreaming(DreamingEvents::DreamComplete {
        agent: context.agent.clone(),
    });

    ticket.db.log_event(&complete, &[])?;
    ticket.broadcast(&complete);

    Ok(Json(context))
}
