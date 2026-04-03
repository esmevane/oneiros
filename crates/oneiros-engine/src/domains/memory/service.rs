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
        let ref_token = RefToken::new(Ref::memory(memory.id));
        Ok(MemoryResponse::MemoryAdded(
            Response::new(memory).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let memory = MemoryRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| MemoryError::NotFound(selector.id))?;
        let ref_token = RefToken::new(Ref::memory(memory.id));
        Ok(MemoryResponse::MemoryDetails(
            Response::new(memory).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListMemories { agent, filters }: &ListMemories,
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

        let listed = MemoryRepo::new(context)
            .list(agent_id.as_deref(), filters)
            .await?;
        Ok(if listed.total == 0 {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(listed.map(|m| {
                let ref_token = RefToken::new(Ref::memory(m.id));
                Response::new(m).with_ref_token(ref_token)
            }))
        })
    }
}
