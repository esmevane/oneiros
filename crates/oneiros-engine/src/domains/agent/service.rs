use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        context: &ProjectLog,
        request: &CreateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let CreateAgent::V1(create) = request;

        // Cross-resource validation: persona must exist
        if PersonaRepo::new(context.scope()?)
            .get(&create.persona)
            .await?
            .is_none()
        {
            return Err(AgentError::PersonaNotFound(create.persona.clone()));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = create.name.normalize_with(&create.persona);

        // Validate name uniqueness
        if AgentRepo::new(context.scope()?)
            .name_exists(&normalized_name)
            .await?
        {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name)
            .persona(create.persona.clone())
            .description(create.description.clone())
            .prompt(create.prompt.clone())
            .build();

        context
            .emit(AgentEvents::AgentCreated(
                AgentCreated::builder_v1()
                    .agent(agent.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(AgentResponse::AgentCreated(
            AgentCreatedResponse::builder_v1()
                .agent(agent)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectLog,
        request: &GetAgent,
    ) -> Result<AgentResponse, AgentError> {
        let GetAgent::V1(lookup) = request;
        let repo = AgentRepo::new(context.scope()?);
        let agent = match &lookup.key {
            ResourceKey::Key(name) => repo
                .get(name)
                .await?
                .ok_or_else(|| AgentError::NotFound(name.clone()))?,
            ResourceKey::Ref(token) => {
                let Ref::V0(resource) = token.inner().clone();
                match resource {
                    Resource::Agent(id) => repo
                        .get_by_id(id)
                        .await?
                        .ok_or(AgentError::NotFoundById(id))?,
                    other => {
                        return Err(AgentError::Resolve(ResolveError::WrongKind {
                            expected: "agent",
                            got: other.label(),
                        }));
                    }
                }
            }
        };
        Ok(AgentResponse::AgentDetails(
            AgentDetailsResponse::builder_v1()
                .agent(agent)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectLog,
        request: &ListAgents,
    ) -> Result<AgentResponse, AgentError> {
        let ListAgents::V1(listing) = request;
        let search_query = SearchQuery::builder_v1()
            .kind(SearchKind::Agent)
            .maybe_query(listing.query.clone())
            .filters(listing.filters)
            .build();

        let results = SearchRepo::new(context.scope()?)
            .search(&search_query, None)
            .await?;

        if results.total == 0 {
            return Ok(AgentResponse::NoAgents);
        }

        let mut ids: Vec<AgentId> = vec![];

        for hit in results.hits {
            if let Ref::V0(Resource::Agent(id)) = hit.resource_ref {
                ids.push(id);
            }
        }

        let items = AgentRepo::new(context.scope()?).get_many(&ids).await?;

        Ok(AgentResponse::Agents(
            AgentsResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }

    pub async fn update(
        context: &ProjectLog,
        request: &UpdateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let UpdateAgent::V1(update) = request;
        let existing = AgentRepo::new(context.scope()?)
            .get(&update.name)
            .await?
            .ok_or_else(|| AgentError::NotFound(update.name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(update.name.clone())
            .persona(update.persona.clone())
            .description(update.description.clone())
            .prompt(update.prompt.clone())
            .build();

        context
            .emit(AgentEvents::AgentUpdated(
                AgentUpdated::builder_v1()
                    .agent(agent.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(AgentResponse::AgentUpdated(
            AgentUpdatedResponse::builder_v1()
                .agent(agent)
                .build()
                .into(),
        ))
    }

    pub async fn remove(
        context: &ProjectLog,
        request: &RemoveAgent,
    ) -> Result<AgentResponse, AgentError> {
        let RemoveAgent::V1(removal) = request;
        let exists = AgentRepo::new(context.scope()?)
            .name_exists(&removal.name)
            .await?;

        if !exists {
            return Err(AgentError::NotFound(removal.name.clone()));
        }

        context
            .emit(AgentEvents::AgentRemoved(
                AgentRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(AgentResponse::AgentRemoved(
            AgentRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
