use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub fn add(
        context: &ProjectContext,
        agent: &AgentName,
        level: LevelName,
        content: Content,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_record = context
            .with_db(|conn| AgentRepo::new(conn).get(agent))
            .map_err(MemoryError::Database)?
            .ok_or_else(|| MemoryError::NotFound(agent.to_string()))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(level)
            .content(content)
            .build();

        context.emit(MemoryEvents::MemoryAdded(memory.clone()));
        Ok(MemoryResponse::MemoryAdded(memory))
    }

    pub fn get(context: &ProjectContext, id: &MemoryId) -> Result<MemoryResponse, MemoryError> {
        let memory = context
            .with_db(|conn| MemoryRepo::new(conn).get(id))
            .map_err(MemoryError::Database)?
            .ok_or_else(|| MemoryError::NotFound(id.to_string()))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub fn list(
        context: &ProjectContext,
        agent: Option<&AgentName>,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_id = agent
            .map(|name| {
                context
                    .with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(MemoryError::Database)?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| MemoryError::NotFound(name.to_string()))
            })
            .transpose()?;

        let memories = context
            .with_db(|conn| MemoryRepo::new(conn).list(agent_id.as_deref()))
            .map_err(MemoryError::Database)?;
        Ok(if memories.is_empty() {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(memories)
        })
    }
}
