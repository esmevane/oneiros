use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub fn add(
        context: &ProjectContext,
        agent: &AgentName,
        texture: TextureName,
        content: Content,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = context
            .with_db(|conn| AgentRepo::new(conn).get(agent))
            .map_err(CognitionError::Database)?
            .ok_or_else(|| CognitionError::NotFound(agent.to_string()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(texture)
            .content(content)
            .build();

        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        context.emit(CognitionEvents::CognitionAdded(cognition.clone()));
        Ok(CognitionResponse::CognitionAdded(CognitionAddedResult {
            id: cognition.id,
            ref_token,
        }))
    }

    pub fn get(
        context: &ProjectContext,
        id: &CognitionId,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = context
            .with_db(|conn| CognitionRepo::new(conn).get(id))
            .map_err(CognitionError::Database)?
            .ok_or_else(|| CognitionError::NotFound(id.to_string()))?;
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
                    .with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(CognitionError::Database)?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| CognitionError::NotFound(name.to_string()))
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
