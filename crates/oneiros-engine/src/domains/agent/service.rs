use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &CreateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let CreateAgent::V1(create) = request;

        // Cross-resource validation: persona must exist
        if PersonaRepo::new(scope)
            .fetch(&create.persona)
            .await?
            .is_none()
        {
            return Err(AgentError::PersonaNotFound(create.persona.clone()));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = create.name.normalize_with(&create.persona);

        // Validate name uniqueness
        if AgentRepo::new(scope).name_exists(&normalized_name).await? {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name.clone())
            .persona(create.persona.clone())
            .description(create.description.clone())
            .prompt(create.prompt.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Agent(AgentEvents::AgentCreated(
                AgentCreated::builder_v1().agent(agent).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = AgentRepo::new(scope)
            .fetch(&normalized_name)
            .await?
            .ok_or(AgentError::NotFound(normalized_name))?;

        Ok(AgentResponse::AgentCreated(
            AgentCreatedResponse::builder_v1()
                .agent(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetAgent,
    ) -> Result<AgentResponse, AgentError> {
        let GetAgent::V1(lookup) = request;
        let repo = AgentRepo::new(scope);
        let agent = match &lookup.key {
            ResourceKey::Key(name) => repo
                .fetch(name)
                .await?
                .ok_or_else(|| AgentError::NotFound(name.clone()))?,
            ResourceKey::Ref(token) => {
                let Ref::V0(resource) = token.inner().clone();
                match resource {
                    Resource::Agent(id) => repo
                        .fetch_by_id(id)
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
        scope: &Scope<AtBookmark>,
        request: &ListAgents,
    ) -> Result<AgentResponse, AgentError> {
        let ListAgents::V1(listing) = request;
        let search_query = SearchQuery::builder_v1()
            .kind(SearchKind::Agent)
            .maybe_query(listing.query.clone())
            .filters(listing.filters)
            .build();

        let results = SearchRepo::new(scope).search(&search_query, None).await?;

        if results.total == 0 {
            return Ok(AgentResponse::NoAgents);
        }

        let mut ids: Vec<AgentId> = vec![];

        for hit in results.hits {
            if let Ref::V0(Resource::Agent(id)) = hit.resource_ref {
                ids.push(id);
            }
        }

        let items = AgentRepo::new(scope).get_many(&ids).await?;

        Ok(AgentResponse::Agents(
            AgentsResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }

    pub async fn update(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &UpdateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let UpdateAgent::V1(update) = request;
        let existing = AgentRepo::new(scope)
            .fetch(&update.name)
            .await?
            .ok_or_else(|| AgentError::NotFound(update.name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(update.name.clone())
            .persona(update.persona.clone())
            .description(update.description.clone())
            .prompt(update.prompt.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Agent(AgentEvents::AgentUpdated(
                AgentUpdated::builder_v1().agent(agent).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        // Read back the updated record. Filter on the fields we just
        // changed so fetch keeps polling until the projection reflects
        // them — otherwise we'd race the projector and return the
        // pre-update record.
        let updated = scope
            .config()
            .fetch
            .eventual(|| async {
                AgentRepo::new(scope).get(&update.name).await.map(|opt| {
                    opt.filter(|agent| {
                        agent.persona == update.persona
                            && agent.description == update.description
                            && agent.prompt == update.prompt
                    })
                })
            })
            .await?
            .ok_or_else(|| AgentError::NotFound(update.name.clone()))?;

        Ok(AgentResponse::AgentUpdated(
            AgentUpdatedResponse::builder_v1()
                .agent(updated)
                .build()
                .into(),
        ))
    }

    pub async fn remove(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveAgent,
    ) -> Result<AgentResponse, AgentError> {
        let RemoveAgent::V1(removal) = request;
        if AgentRepo::new(scope).fetch(&removal.name).await?.is_none() {
            return Err(AgentError::NotFound(removal.name.clone()));
        }

        let new_event = NewEvent::builder()
            .data(Events::Agent(AgentEvents::AgentRemoved(
                AgentRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { AgentRepo::new(scope).get(&removal.name).await })
            .await?;

        Ok(AgentResponse::AgentRemoved(
            AgentRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
