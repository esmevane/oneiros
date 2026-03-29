use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub async fn add(
        context: &ProjectContext,
        agent: AgentName,
        level: LevelName,
        content: Content,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_record = AgentRepo::new(&context.db()?)
            .get(&agent)?
            .ok_or_else(|| MemoryError::AgentNotFound(agent.clone()))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(level)
            .content(content)
            .build();

        context
            .emit(MemoryEvents::MemoryAdded(memory.clone()))
            .await?;
        Ok(MemoryResponse::MemoryAdded(memory))
    }

    pub fn get(context: &ProjectContext, id: &MemoryId) -> Result<MemoryResponse, MemoryError> {
        let memory = MemoryRepo::new(&context.db()?)
            .get(id)?
            .ok_or_else(|| MemoryError::NotFound(*id))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub fn list(
        context: &ProjectContext,
        agent: Option<AgentName>,
    ) -> Result<MemoryResponse, MemoryError> {
        let db = context.db()?;

        let agent_id = agent
            .map(|name| {
                AgentRepo::new(&db)
                    .get(&name)?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| MemoryError::AgentNotFound(name.clone()))
            })
            .transpose()?;

        let memories = MemoryRepo::new(&db).list(agent_id.as_deref())?;
        Ok(if memories.is_empty() {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(memories)
        })
    }
}
