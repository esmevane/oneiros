use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::handlers::dream::collector::{DreamCollector, DreamConfig};
use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let woke = Events::Lifecycle(LifecycleEvents::Woke {
        name: agent.name.clone(),
    });
    ticket.db.log_event(&woke, &[])?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let context = DreamCollector::new(&ticket.db, DreamConfig::default()).collect(&agent)?;

    let complete = Events::Dreaming(DreamingEvents::DreamComplete {
        agent: context.agent.clone(),
    });

    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
