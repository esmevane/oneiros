use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<StatusCode, Error> {
    ticket
        .service()
        .dispatch_lifecycle(LifecycleRequests::Recede(RecedeRequest { agent }))?;

    Ok(StatusCode::OK)
}
