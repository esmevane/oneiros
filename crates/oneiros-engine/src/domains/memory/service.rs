use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub async fn add(
        context: &ProjectContext,
        agent: AgentName,
        level: LevelName,
        content: Content,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_record = AgentRepo::new(context)
            .get(&agent)
            .await?
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

    pub async fn get(
        context: &ProjectContext,
        id: &MemoryId,
    ) -> Result<MemoryResponse, MemoryError> {
        let memory = MemoryRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| MemoryError::NotFound(*id))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub async fn list(
        context: &ProjectContext,
        agent: Option<AgentName>,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(&name)
                    .await?
                    .ok_or_else(|| MemoryError::AgentNotFound(name.clone()))?;
                Some(record.id.to_string())
            }
            None => None,
        };

        let memories = MemoryRepo::new(context).list(agent_id.as_deref()).await?;
        Ok(if memories.is_empty() {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(memories)
        })
    }
}
