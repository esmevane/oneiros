use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
) -> Result<Json<Vec<Identity<AgentId, Agent>>>, Error> {
    let agents = ticket.db.list_agents()?;

    Ok(Json(agents))
}
