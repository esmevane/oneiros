use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub fn add(
        ctx: &ProjectContext,
        agent: String,
        texture: String,
        content: String,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = ctx
            .with_db(|conn| AgentRepo::new(conn).get(&agent))
            .map_err(CognitionError::Database)?
            .ok_or_else(|| CognitionError::NotFound(agent))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(texture)
            .content(content)
            .build();

        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        ctx.emit(CognitionEvents::CognitionAdded(cognition.clone()));
        Ok(CognitionResponse::CognitionAdded(CognitionAddedResult {
            id: cognition.id,
            ref_token,
        }))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<CognitionResponse, CognitionError> {
        let cognition = ctx
            .with_db(|conn| CognitionRepo::new(conn).get(id))
            .map_err(CognitionError::Database)?
            .ok_or_else(|| CognitionError::NotFound(id.to_string()))?;
        Ok(CognitionResponse::CognitionDetails(cognition))
    }

    pub fn list(
        ctx: &ProjectContext,
        agent: Option<&str>,
        texture: Option<&str>,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_id = agent
            .map(|name| {
                ctx.with_db(|conn| AgentRepo::new(conn).get(name))
                    .map_err(CognitionError::Database)?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| CognitionError::NotFound(name.to_string()))
            })
            .transpose()?;

        let cognitions = ctx
            .with_db(|conn| CognitionRepo::new(conn).list(agent_id.as_deref(), texture))
            .map_err(CognitionError::Database)?;
        Ok(if cognitions.is_empty() {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(cognitions)
        })
    }
}
