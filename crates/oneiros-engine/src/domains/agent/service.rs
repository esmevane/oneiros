use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        context: &ProjectContext,
        request: &CreateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let CreateAgent::V1(create) = request;

        // Cross-resource validation: persona must exist
        if PersonaRepo::new(context)
            .get(&create.persona)
            .await?
            .is_none()
        {
            return Err(AgentError::PersonaNotFound(create.persona.clone()));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = create.name.normalize_with(&create.persona);

        // Validate name uniqueness
        if AgentRepo::new(context)
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
        context: &ProjectContext,
        request: &GetAgent,
    ) -> Result<AgentResponse, AgentError> {
        let GetAgent::V1(lookup) = request;
        let repo = AgentRepo::new(context);
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
        context: &ProjectContext,
        request: &ListAgents,
    ) -> Result<AgentResponse, AgentError> {
        let ListAgents::V1(listing) = request;
        let listed = AgentRepo::new(context).list(&listing.filters).await?;

        if listed.total == 0 {
            Ok(AgentResponse::NoAgents)
        } else {
            Ok(AgentResponse::Agents(
                AgentsResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn update(
        context: &ProjectContext,
        request: &UpdateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let UpdateAgent::V1(update) = request;
        let existing = AgentRepo::new(context)
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
        context: &ProjectContext,
        request: &RemoveAgent,
    ) -> Result<AgentResponse, AgentError> {
        let RemoveAgent::V1(removal) = request;
        let exists = AgentRepo::new(context).name_exists(&removal.name).await?;

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
