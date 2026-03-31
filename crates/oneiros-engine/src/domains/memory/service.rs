use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub async fn add(
        context: &ProjectContext,
        AddMemory {
            agent,
            level,
            content,
        }: &AddMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_record = AgentRepo::new(context)
            .get(agent)
            .await?
            .ok_or_else(|| MemoryError::AgentNotFound(agent.clone()))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(level.clone())
            .content(content.clone())
            .build();

        context
            .emit(MemoryEvents::MemoryAdded(memory.clone()))
            .await?;
        Ok(MemoryResponse::MemoryAdded(memory))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let memory = MemoryRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| MemoryError::NotFound(selector.id))?;
        Ok(MemoryResponse::MemoryDetails(memory))
    }

    pub async fn list(
        context: &ProjectContext,
        ListMemories { agent }: &ListMemories,
    ) -> Result<MemoryResponse, MemoryError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(name)
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
