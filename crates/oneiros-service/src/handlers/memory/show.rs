use axum::{Json, extract::Path};
use oneiros_model::{Content, LevelName, Memory, MemoryId};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<Memory>, Error> {
    let (mid, agent_id, level, content, created_at) = ticket
        .db
        .get_memory(id.to_string())?
        .ok_or(NotFound::Memory(id))?;

    Ok(Json(Memory {
        id: mid.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        level: LevelName::new(level),
        content: Content::new(content),
        created_at: created_at.parse().unwrap_or_default(),
    }))
}
