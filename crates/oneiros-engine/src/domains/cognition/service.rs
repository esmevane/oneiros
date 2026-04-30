use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub async fn add(
        context: &ProjectLog,
        request: &AddCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let AddCognition::V1(addition) = request;
        let agent_record = AgentRepo::new(context.scope()?)
            .get(&addition.agent)
            .await?
            .ok_or_else(|| CognitionError::AgentNotFound(addition.agent.clone()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(addition.texture.clone())
            .content(addition.content.clone())
            .build();

        context
            .emit(CognitionEvents::CognitionAdded(
                CognitionAdded::builder_v1()
                    .cognition(cognition.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(CognitionResponse::CognitionAdded(
            CognitionAddedResponse::builder_v1()
                .cognition(cognition)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectLog,
        request: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let GetCognition::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let cognition = CognitionRepo::new(context.scope()?)
            .get(&id)
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
        context: &ProjectLog,
        request: &ListCognitions,
    ) -> Result<CognitionResponse, CognitionError> {
        let ListCognitions::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(context.scope()?)
                    .get(name)
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

        let results = SearchRepo::new(context.scope()?)
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
        let items = CognitionRepo::new(context.scope()?).get_many(&ids).await?;

        Ok(CognitionResponse::Cognitions(
            CognitionsResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }
}
