use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub fn add(
        ctx: &ProjectContext,
        agent: String,
        level: String,
        content: String,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_record = ctx
            .with_db(|conn| AgentRepo::new(conn).get(&agent))
            .map_err(MemoryError::Database)?
            .ok_or_else(|| MemoryError::NotFound(agent))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(level)
            .content(content)
            .build();

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
        let agent_id = agent
            .map(|name| {
                ctx.with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(MemoryError::Database)?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| MemoryError::NotFound(name.to_string()))
            })
            .transpose()?;

        let memories = ctx
            .with_db(|conn| MemoryRepo::new(conn).list(agent_id.as_deref()))
            .map_err(MemoryError::Database)?;
        Ok(if memories.is_empty() {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(memories)
        })
    }
}
