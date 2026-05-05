use crate::*;

pub struct CognitionService;

impl CognitionService {
    /// Record a cognition by dispatching `CognitionAdded` through the
    /// bus and reading the eventually-consistent record back via fetch.
    ///
    /// No phantom state — the response carries whatever the projection
    /// has seen, never a synthesised record. If the fetch window
    /// expires before the projection catches up, this surfaces as
    /// `CognitionError::NotFound`.
    pub async fn add(
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
        let id = cognition.id;

        let new_event = NewEvent::builder()
            .data(Events::Cognition(CognitionEvents::CognitionAdded(
                CognitionAdded::builder_v1()
                    .cognition(cognition)
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = CognitionRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(CognitionError::NotFound(id))?;

        Ok(CognitionResponse::CognitionAdded(
            CognitionAddedResponse::builder_v1()
                .cognition(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
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

    pub async fn list(
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
