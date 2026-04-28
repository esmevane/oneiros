use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub async fn add(
        context: &ProjectContext,
        request: &AddMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let AddMemory::V1(addition) = request;
        let agent_record = AgentRepo::new(context)
            .get(&addition.agent)
            .await?
            .ok_or_else(|| MemoryError::AgentNotFound(addition.agent.clone()))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(addition.level.clone())
            .content(addition.content.clone())
            .build();

        context
            .emit(MemoryEvents::MemoryAdded(
                MemoryAdded::builder_v1()
                    .memory(memory.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(MemoryResponse::MemoryAdded(
            MemoryAddedResponse::builder_v1()
                .memory(memory)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let GetMemory::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let memory = MemoryRepo::new(context)
            .get(&id)
            .await?
            .ok_or(MemoryError::NotFound(id))?;
        Ok(MemoryResponse::MemoryDetails(
            MemoryDetailsResponse::builder_v1()
                .memory(memory)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        request: &ListMemories,
    ) -> Result<MemoryResponse, MemoryError> {
        let ListMemories::V1(listing) = request;
        let agent_id = match &listing.agent {
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
            .list(agent_id.as_deref(), &listing.filters)
            .await?;
        Ok(if listed.total == 0 {
            MemoryResponse::NoMemories
        } else {
            MemoryResponse::Memories(
                MemoriesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            )
        })
    }
}
