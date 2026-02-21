use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<AgentName, AgentLink>>,
) -> Result<Json<Record<Identity<AgentId, Agent>>>, Error> {
    let agent = ticket
        .db
        .get_agent_by_key(&key)?
        .ok_or(NotFound::Agent(key))?;

    let record = Record::new(agent)?;
    Ok(Json(record))
}
