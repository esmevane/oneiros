use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
) -> Result<StatusCode, Error> {
    ticket
        .db
        .get_agent(&name)?
        .ok_or(NotFound::Agent(Key::Id(name.clone())))?;

    let receded = Events::Lifecycle(LifecycleEvents::Receded { name: name.clone() });
    ticket.db.log_event(&receded, &[])?;

    let removed = Events::Agent(AgentEvents::AgentRemoved { name });
    ticket
        .db
        .log_event(&removed, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
