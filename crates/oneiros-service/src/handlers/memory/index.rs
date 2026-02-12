use axum::{Json, extract::Query};
use oneiros_model::{AgentName, Content, LevelName, Memory};
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub level: Option<LevelName>,
}

fn to_memory(row: (String, String, String, String, String)) -> Memory {
    let (id, agent_id, level, content, created_at) = row;
    Memory {
        id: id.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        level: LevelName::new(level),
        content: Content::new(content),
        created_at: created_at.parse().unwrap_or_default(),
    }
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Memory>>, Error> {
    let rows = match (params.agent, params.level) {
        (Some(agent_name), Some(level)) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket
                .db
                .get_level(&level)?
                .ok_or(NotFound::Level(level.clone()))?;

            ticket.db.list_memories_by_agent_and_level(&id, &level)?
        }
        (Some(agent_name), None) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket.db.list_memories_by_agent(&id)?
        }
        (None, Some(level)) => {
            ticket
                .db
                .get_level(&level)?
                .ok_or(NotFound::Level(level.clone()))?;

            ticket.db.list_memories_by_level(&level)?
        }
        (None, None) => ticket.db.list_memories()?,
    };

    let memories = rows.into_iter().map(to_memory).collect::<Vec<_>>();

    Ok(Json(memories))
}
