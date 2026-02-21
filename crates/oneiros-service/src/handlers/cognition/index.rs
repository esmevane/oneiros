use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub texture: Option<TextureName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Record<CognitionId, Cognition>>>, Error> {
    let cognitions = match (params.agent, params.texture) {
        (Some(agent_name), Some(texture)) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket
                .db
                .get_texture(&texture)?
                .ok_or(NotFound::Texture(texture.clone()))?;

            ticket
                .db
                .list_cognitions_by_agent_and_texture(agent.id.to_string(), &texture)?
        }
        (Some(agent_name), None) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket.db.list_cognitions_by_agent(agent.id.to_string())?
        }
        (None, Some(texture)) => {
            ticket
                .db
                .get_texture(&texture)?
                .ok_or(NotFound::Texture(texture.clone()))?;

            ticket.db.list_cognitions_by_texture(&texture)?
        }
        (None, None) => ticket.db.list_cognitions()?,
    };

    Ok(Json(cognitions))
}
