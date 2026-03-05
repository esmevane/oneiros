use axum::{Json, extract::Query};
use oneiros_model::*;
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
) -> Result<Json<MemoryResponses>, Error> {
    let memories = ticket.service().list_memories(params.agent, params.level)?;

    Ok(Json(memories))
}
