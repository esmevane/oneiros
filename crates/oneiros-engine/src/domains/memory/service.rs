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
            .with_db(|conn| AgentRepo::new(conn).get(agent))?
            .ok_or_else(|| MemoryError::AgentNotFound(agent.clone()))?;

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
            .with_db(|conn| MemoryRepo::new(conn).get(id))?
            .ok_or_else(|| MemoryError::NotFound(*id))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub fn list(
        context: &ProjectContext,
        agent: Option<&AgentName>,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_id = agent
            .map(|name| {
                context
                    .with_db(|conn| AgentRepo::new(conn).get(name))?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| MemoryError::AgentNotFound(name.clone()))
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
