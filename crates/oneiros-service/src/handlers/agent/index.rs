use axum::Json;
use oneiros_model::Agent;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Agent>>, Error> {
    let agents = ticket.db.list_agents()?;

    Ok(Json(agents))
}
