use chrono::Utc;

use crate::*;

pub struct CognitionService;

impl CognitionService {
    pub fn add(
        ctx: &ProjectContext,
        agent: String,
        texture: String,
        content: String,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = Cognition {
            id: CognitionId::new(),
            agent_id: agent,
            texture,
            content,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit(CognitionEvents::CognitionAdded(cognition.clone()));
        Ok(CognitionResponse::Added(cognition))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<CognitionResponse, CognitionError> {
        let cognition = ctx
            .with_db(|conn| CognitionRepo::new(conn).get(id))
            .map_err(CognitionError::Database)?
            .ok_or_else(|| CognitionError::NotFound(id.to_string()))?;
        Ok(CognitionResponse::Found(cognition))
    }

    pub fn list(
        ctx: &ProjectContext,
        agent: Option<&str>,
        texture: Option<&str>,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognitions = ctx
            .with_db(|conn| CognitionRepo::new(conn).list(agent, texture))
            .map_err(CognitionError::Database)?;
        Ok(CognitionResponse::Listed(cognitions))
    }
}
