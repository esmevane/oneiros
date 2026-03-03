use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<Agent>, Error> {
    let agent = ticket.service().sense(&agent_name)?;

    Ok(Json(agent))
}
