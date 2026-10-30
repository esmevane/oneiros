use crate::*;

pub(crate) struct CognitionService;

impl CognitionService {
    /// Record a cognition by telling the bus about `CognitionAdded` and waiting
    /// until the event is persisted in the log.
    ///
    /// The response carries the record from the event itself — the same
    /// payload the projection applies. Once the event is persisted the
    /// request succeeds; reading the projection back here could fail a
    /// write that already happened, and the client's retry would
    /// duplicate it.
    pub(crate) async fn add(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &AddCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let AddCognition::V1(addition) = request;
        let agent_record = AgentRepo::new(scope)
            .fetch(&addition.agent)
            .await?
            .ok_or_else(|| CognitionError::AgentNotFound(addition.agent.clone()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(addition.texture.clone())
            .content(addition.content.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Cognition(CognitionEvents::CognitionAdded(
                CognitionAdded::builder_v1()
                    .cognition(cognition.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(CognitionResponse::CognitionAdded(
            CognitionAddedResponse::builder_v1()
                .cognition(cognition)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let GetCognition::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let cognition = CognitionRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(CognitionError::NotFound(id))?;
        Ok(CognitionResponse::CognitionDetails(
            CognitionDetailsResponse::builder_v1()
                .cognition(cognition)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtBookmark>,
        request: &ListCognitions,
    ) -> Result<CognitionResponse, CognitionError> {
        let ListCognitions::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(scope)
                    .fetch(name)
                    .await?
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let search_query = SearchQuery::builder_v1()
            .kind(SearchKind::Cognition)
            .maybe_texture(listing.texture.clone())
            .maybe_query(listing.query.clone())
            .filters(listing.filters)
            .build();

        let results = SearchRepo::new(scope)
            .search(&search_query, agent_id.as_ref())
            .await?;

        if results.total == 0 {
            return Ok(CognitionResponse::NoCognitions);
        }

        let ids: Vec<CognitionId> = results
            .hits
            .iter()
            .filter_map(|hit| match &hit.resource_ref {
                Ref::V0(Resource::Cognition(id)) => Some(*id),
                _ => None,
            })
            .collect();
        let items = CognitionRepo::new(scope).get_many(&ids).await?;

        Ok(CognitionResponse::Cognitions(
            CognitionsResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }
}
