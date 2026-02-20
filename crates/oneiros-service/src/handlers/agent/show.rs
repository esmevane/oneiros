use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Identity<AgentId, Agent>>>, Error> {
    let by_name = ticket.db.get_agent(AgentName::new(&identifier))?;

    let agent = if let Some(a) = by_name {
        a
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_agent_by_link(link.to_string())?
            .ok_or(NotFound::Agent(AgentName::new(&identifier)))?
    } else {
        return Err(NotFound::Agent(AgentName::new(&identifier)).into());
    };

    let record = Record::new(agent)?;
    Ok(Json(record))
}
