use axum::{Json, extract::Query};
use oneiros_model::{AgentName, Identity, Key, LevelName, Memory, MemoryId};
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub level: Option<LevelName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Identity<MemoryId, Memory>>>, Error> {
    let memories = match (params.agent, params.level) {
        (Some(agent_name), Some(level)) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(Key::Id(agent_name)))?;

            ticket
                .db
                .get_level(&level)?
                .ok_or(NotFound::Level(Key::Id(level.clone())))?;

            ticket
                .db
                .list_memories_by_agent_and_level(&agent.id, &level)?
        }
        (Some(agent_name), None) => {
            let agent = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(Key::Id(agent_name)))?;

            ticket.db.list_memories_by_agent(&agent.id)?
        }
        (None, Some(level)) => {
            ticket
                .db
                .get_level(&level)?
                .ok_or(NotFound::Level(Key::Id(level.clone())))?;

            ticket.db.list_memories_by_level(&level)?
        }
        (None, None) => ticket.db.list_memories()?,
    };

    Ok(Json(memories))
}
