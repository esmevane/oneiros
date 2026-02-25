use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub sensation: Option<SensationName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Experience>>, Error> {
    let experiences = match (params.agent, params.sensation) {
        (Some(agent_name), Some(sensation)) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket
                .db
                .get_sensation(&sensation)?
                .ok_or(NotFound::Sensation(sensation.clone()))?;

            ticket
                .db
                .list_experiences_by_agent(agent.id.to_string())?
                .into_iter()
                .filter(|exp| exp.sensation == sensation)
                .collect()
        }
        (Some(agent_name), None) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket.db.list_experiences_by_agent(agent.id.to_string())?
        }
        (None, Some(sensation)) => {
            ticket
                .db
                .get_sensation(&sensation)?
                .ok_or(NotFound::Sensation(sensation.clone()))?;

            ticket.db.list_experiences_by_sensation(&sensation)?
        }
        (None, None) => ticket.db.list_experiences()?,
    };

    Ok(Json(experiences))
}
