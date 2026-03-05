use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<AgentName>,
) -> Result<StatusCode, Error> {
    ticket.service().remove_agent(name)?;

    Ok(StatusCode::OK)
}
