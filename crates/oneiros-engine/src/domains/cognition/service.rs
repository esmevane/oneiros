use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub async fn add(
        context: &ProjectContext,
        request: &AddCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let AddCognition::V1(addition) = request;
        let agent_record = AgentRepo::new(context)
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
        context: &ProjectContext,
        request: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let GetCognition::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let cognition = CognitionRepo::new(context)
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
        context: &ProjectContext,
        request: &ListCognitions,
    ) -> Result<CognitionResponse, CognitionError> {
        let ListCognitions::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(name)
                    .await?
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let listed = CognitionRepo::new(context)
            .list(
                agent_id.as_ref(),
                listing.texture.as_ref(),
                &listing.filters,
            )
            .await?;
        Ok(if listed.total == 0 {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(
                CognitionsResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            )
        })
    }
}
