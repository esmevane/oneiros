use axum::{extract::Path, http::StatusCode};
use oneiros_model::AgentName;
use oneiros_protocol::{AgentEvents, Events};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
) -> Result<StatusCode, Error> {
    let event = Events::Agent(AgentEvents::AgentRemoved { name });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
