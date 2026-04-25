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
        let id = selector.key.resolve()?;
        let memory = MemoryRepo::new(context)
            .get(&id)
            .await?
            .ok_or(MemoryError::NotFound(id))?;
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
                Some(record.id)
            }
            None => None,
        };

        let search_query = SearchQuery::builder()
            .kind(SearchKind::Memory)
            .filters(*filters)
            .build();

        let results = SearchRepo::new(context)
            .search(&search_query, agent_id.as_ref())
            .await?;

        if results.total == 0 {
            return Ok(MemoryResponse::NoMemories);
        }

        let ids: Vec<MemoryId> = results
            .hits
            .iter()
            .filter_map(|hit| match hit.resource_ref() {
                Ref::V0(Resource::Memory(id)) => Some(*id),
                _ => None,
            })
            .collect();
        let items = MemoryRepo::new(context).get_many(&ids).await?;

        Ok(MemoryResponse::Memories(
            Listed::new(items, results.total).map(|m| {
                let ref_token = RefToken::new(Ref::memory(m.id));
                Response::new(m).with_ref_token(ref_token)
            }),
        ))
    }
}
