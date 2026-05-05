use crate::*;

pub struct MemoryService;

impl MemoryService {
    pub async fn add(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &AddMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let AddMemory::V1(addition) = request;
        let agent_record = AgentRepo::new(scope)
            .fetch(&addition.agent)
            .await?
            .ok_or_else(|| MemoryError::AgentNotFound(addition.agent.clone()))?;

        let memory = Memory::builder()
            .agent_id(agent_record.id)
            .level(addition.level.clone())
            .content(addition.content.clone())
            .build();
        let id = memory.id;

        let new_event = NewEvent::builder()
            .data(Events::Memory(MemoryEvents::MemoryAdded(
                MemoryAdded::builder_v1().memory(memory).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = MemoryRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(MemoryError::NotFound(id))?;

        Ok(MemoryResponse::MemoryAdded(
            MemoryAddedResponse::builder_v1()
                .memory(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetMemory,
    ) -> Result<MemoryResponse, MemoryError> {
        let GetMemory::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let memory = MemoryRepo::new(scope)
            .fetch(&id)
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
        scope: &Scope<AtBookmark>,
        request: &ListMemories,
    ) -> Result<MemoryResponse, MemoryError> {
        let ListMemories::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(scope)
                    .fetch(name)
                    .await?
                    .ok_or_else(|| MemoryError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let search_query = SearchQuery::builder_v1()
            .kind(SearchKind::Memory)
            .maybe_level(listing.level.clone())
            .maybe_query(listing.query.clone())
            .filters(listing.filters)
            .build();

        let results = SearchRepo::new(scope)
            .search(&search_query, agent_id.as_ref())
            .await?;

        if results.total == 0 {
            return Ok(MemoryResponse::NoMemories);
        }

        let ids: Vec<MemoryId> = results
            .hits
            .iter()
            .filter_map(|hit| match &hit.resource_ref {
                Ref::V0(Resource::Memory(id)) => Some(*id),
                _ => None,
            })
            .collect();
        let items = MemoryRepo::new(scope).get_many(&ids).await?;

        Ok(MemoryResponse::Memories(
            MemoriesResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }
}
