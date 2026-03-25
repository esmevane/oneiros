use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub async fn add(
        context: &ProjectContext,
        agent: AgentName,
        texture: TextureName,
        content: Content,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = AgentRepo::new(context)
            .get(&agent)
            .await?
            .ok_or_else(|| CognitionError::AgentNotFound(agent.clone()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(texture)
            .content(content)
            .build();

        context
            .emit(CognitionEvents::CognitionAdded(cognition.clone()))
            .await?;
        Ok(CognitionResponse::CognitionAdded(cognition))
    }

    pub async fn get(
        context: &ProjectContext,
        id: &CognitionId,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = CognitionRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| CognitionError::NotFound(*id))?;
        Ok(CognitionResponse::CognitionDetails(cognition))
    }

    pub async fn list(
        context: &ProjectContext,
        agent: Option<AgentName>,
        texture: Option<TextureName>,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(&name)
                    .await?
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let cognitions = CognitionRepo::new(context)
            .list(agent_id.as_ref(), texture.as_ref())
            .await?;
        Ok(if cognitions.is_empty() {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(cognitions)
        })
    }
}
