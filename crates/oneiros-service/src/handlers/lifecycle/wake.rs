use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<LifecycleResponses>, Error> {
    let response = ticket.service().wake(&agent_name)?;

    Ok(Json(response))
}
