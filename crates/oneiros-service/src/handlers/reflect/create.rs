use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<ReflectingResponses>, Error> {
    let response = ticket.service().reflect(&agent_name)?;

    Ok(Json(response))
}
