use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub async fn add(
        context: &ProjectContext,
        agent: &AgentName,
        texture: TextureName,
        content: Content,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = context
            .with_db(|conn| AgentRepo::new(conn).get(agent))?
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

    pub fn get(
        context: &ProjectContext,
        id: &CognitionId,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = context
            .with_db(|conn| CognitionRepo::new(conn).get(id))?
            .ok_or_else(|| CognitionError::NotFound(*id))?;
        Ok(CognitionResponse::CognitionDetails(cognition))
    }

    pub fn list(
        context: &ProjectContext,
        agent: Option<&AgentName>,
        texture: Option<&TextureName>,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_id = agent
            .map(|name| {
                context
                    .with_db(|conn| AgentRepo::new(conn).get(name))?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))
            })
            .transpose()?;

        let texture_str = texture.map(|t| t.to_string());

        let cognitions = context
            .with_db(|conn| {
                CognitionRepo::new(conn).list(agent_id.as_deref(), texture_str.as_deref())
            })
            .map_err(CognitionError::Database)?;
        Ok(if cognitions.is_empty() {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(cognitions)
        })
    }
}
