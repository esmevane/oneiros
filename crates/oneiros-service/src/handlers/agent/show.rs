use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<AgentName>,
) -> Result<Json<Agent>, Error> {
    let agent = ticket.service().get_agent(&given_name)?;

    Ok(Json(agent))
}
