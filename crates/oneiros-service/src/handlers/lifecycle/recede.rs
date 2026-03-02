use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
) -> Result<StatusCode, Error> {
    ticket
        .db
        .get_agent(&name)?
        .ok_or(NotFound::Agent(name.clone()))?;

    let receded = Events::Lifecycle(LifecycleEvents::Receded { name: name.clone() });
    ticket.db.log_event(&receded, &[])?;
    ticket.broadcast(&receded);

    let removed = Events::Agent(AgentEvents::AgentRemoved { name });
    ticket.db.log_event(&removed, projections::BRAIN)?;
    ticket.broadcast(&removed);

    Ok(StatusCode::OK)
}
