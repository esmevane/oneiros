use axum::{Json, extract::Query};
use oneiros_model::{AgentName, Cognition, Content, TextureName};
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub texture: Option<TextureName>,
}

fn to_cognition(row: (String, String, String, String, String)) -> Cognition {
    let (id, agent_id, texture, content, created_at) = row;
    Cognition {
        id: id.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        texture: TextureName::new(texture),
        content: Content::new(content),
        created_at: created_at.parse().unwrap_or_default(),
    }
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Cognition>>, Error> {
    let rows = match (params.agent, params.texture) {
        (Some(agent_name), Some(texture)) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket
                .db
                .get_texture(&texture)?
                .ok_or(NotFound::Texture(texture.clone()))?;

            ticket
                .db
                .list_cognitions_by_agent_and_texture(&id, &texture)?
        }
        (Some(agent_name), None) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket.db.list_cognitions_by_agent(&id)?
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

    let cognitions = rows.into_iter().map(to_cognition).collect::<Vec<_>>();

    Ok(Json(cognitions))
}
