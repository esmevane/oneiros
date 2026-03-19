use chrono::Utc;

use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub fn add(
        ctx: &ProjectContext,
        agent: String,
        level: String,
        content: String,
    ) -> Result<MemoryResponse, MemoryError> {
        let memory = Memory {
            id: MemoryId::new(),
            agent_id: agent,
            level,
            content,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit(MemoryEvents::MemoryAdded(memory.clone()));
        Ok(MemoryResponse::MemoryAdded(memory))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<MemoryResponse, MemoryError> {
        let memory = ctx
            .with_db(|conn| MemoryRepo::new(conn).get(id))
            .map_err(MemoryError::Database)?
            .ok_or_else(|| MemoryError::NotFound(id.to_string()))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub fn list(ctx: &ProjectContext, agent: Option<&str>) -> Result<MemoryResponse, MemoryError> {
        let memories = ctx
            .with_db(|conn| MemoryRepo::new(conn).list(agent))
            .map_err(MemoryError::Database)?;
        Ok(if memories.is_empty() {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(memories)
        })
    }
}
