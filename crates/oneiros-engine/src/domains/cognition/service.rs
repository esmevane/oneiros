use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub async fn add(
        context: &ProjectContext,
        AddCognition {
            agent,
            texture,
            content,
        }: &AddCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = AgentRepo::new(context)
            .get(agent)
            .await?
            .ok_or_else(|| CognitionError::AgentNotFound(agent.clone()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(texture.clone())
            .content(content.clone())
            .build();

        context
            .emit(CognitionEvents::CognitionAdded(cognition.clone()))
            .await?;
        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        Ok(CognitionResponse::CognitionAdded(
            Response::new(cognition).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let id = selector.key.resolve()?;
        let cognition = CognitionRepo::new(context)
            .get(&id)
            .await?
            .ok_or(CognitionError::NotFound(id))?;
        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        Ok(CognitionResponse::CognitionDetails(
            Response::new(cognition).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListCognitions {
            agent,
            texture,
            filters,
        }: &ListCognitions,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(name)
                    .await?
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let search_query = SearchQuery::builder()
            .kind(SearchKind::Cognition)
            .maybe_texture(texture.clone())
            .filters(*filters)
            .build();

        let results = SearchRepo::new(context)
            .search(&search_query, agent_id.as_ref())
            .await?;

        if results.total == 0 {
            return Ok(CognitionResponse::NoCognitions);
        }

        let repo = CognitionRepo::new(context);
        let mut items = Vec::with_capacity(results.hits.len());
        for hit in &results.hits {
            let Ref::V0(Resource::Cognition(id)) = &hit.resource_ref else {
                continue;
            };
            if let Some(cognition) = repo.get(id).await? {
                items.push(cognition);
            }
        }

        Ok(CognitionResponse::Cognitions(
            Listed::new(items, results.total).map(|c| {
                let ref_token = RefToken::new(Ref::cognition(c.id));
                Response::new(c).with_ref_token(ref_token)
            }),
        ))
    }
}
