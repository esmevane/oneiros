use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        context: &ProjectContext,
        CreateAgent {
            name,
            persona,
            description,
            prompt,
        }: &CreateAgent,
    ) -> Result<AgentResponse, AgentError> {
        // Cross-resource validation: persona must exist
        //
        if PersonaRepo::new(context).get(persona).await?.is_none() {
            return Err(AgentError::PersonaNotFound(persona.clone()));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = name.normalize_with(persona);

        // Validate name uniqueness
        if AgentRepo::new(context)
            .name_exists(&normalized_name)
            .await?
        {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name.clone())
            .persona(persona.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();

        context.emit(AgentEvents::AgentCreated(agent)).await?;

        Ok(AgentResponse::AgentCreated(normalized_name))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetAgent,
    ) -> Result<AgentResponse, AgentError> {
        let repo = AgentRepo::new(context);
        let agent = match &selector.key {
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
        let ref_token = RefToken::new(Ref::agent(agent.id));
        Ok(AgentResponse::AgentDetails(
            Response::new(agent).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListAgents { filters }: &ListAgents,
    ) -> Result<AgentResponse, AgentError> {
        let search_query = SearchQuery::builder()
            .kind(SearchKind::Agent)
            .filters(*filters)
            .build();

        let results = SearchRepo::new(context).search(&search_query, None).await?;

        if results.total == 0 {
            return Ok(AgentResponse::NoAgents);
        }

        let mut ids: Vec<AgentId> = vec![];

        for hit in results.hits {
            if let Ref::V0(Resource::Agent(id)) = hit.resource_ref() {
                ids.push(*id);
            }
        }

        let items = AgentRepo::new(context).get_many(&ids).await?;

        Ok(AgentResponse::Agents(
            Listed::new(items, results.total).map(|agent| {
                let ref_token = RefToken::new(Ref::agent(agent.id));
                Response::new(agent).with_ref_token(ref_token)
            }),
        ))
    }

    pub async fn update(
        context: &ProjectContext,
        UpdateAgent {
            name,
            persona,
            description,
            prompt,
        }: &UpdateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let existing = AgentRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(name.clone())
            .persona(persona.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();

        context.emit(AgentEvents::AgentUpdated(agent)).await?;

        Ok(AgentResponse::AgentUpdated(name.clone()))
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveAgent,
    ) -> Result<AgentResponse, AgentError> {
        let exists = AgentRepo::new(context).name_exists(&selector.name).await?;

        if !exists {
            return Err(AgentError::NotFound(selector.name.clone()));
        }

        context
            .emit(AgentEvents::AgentRemoved(AgentRemoved {
                name: selector.name.clone(),
            }))
            .await?;

        Ok(AgentResponse::AgentRemoved(selector.name.clone()))
    }
}
