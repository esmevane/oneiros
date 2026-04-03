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
        Ok(CognitionResponse::CognitionAdded(cognition))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = CognitionRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| CognitionError::NotFound(selector.id))?;
        Ok(CognitionResponse::CognitionDetails(cognition))
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

        let listed = CognitionRepo::new(context)
            .list(agent_id.as_ref(), texture.as_ref(), filters)
            .await?;
        Ok(if listed.total == 0 {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(listed)
        })
    }
}
