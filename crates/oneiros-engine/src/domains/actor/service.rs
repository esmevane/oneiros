use chrono::Utc;

use crate::*;

pub struct ActorService;

impl ActorService {
    pub async fn create(
        context: &SystemContext,
        tenant_id: TenantId,
        name: ActorName,
    ) -> Result<ActorResponse, ActorError> {
        let actor = Actor {
            id: ActorId::new(),
            tenant_id: tenant_id.to_string(),
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        context
            .emit(ActorEvents::ActorCreated(actor.clone()))
            .await?;
        Ok(ActorResponse::Created(actor))
    }

    pub fn get(context: &SystemContext, id: &ActorId) -> Result<ActorResponse, ActorError> {
        let actor = context
            .with_db(|conn| ActorRepo::new(conn).get(id))?
            .ok_or_else(|| ActorError::NotFound(id.clone()))?;
        Ok(ActorResponse::Found(actor))
    }

    pub fn list(context: &SystemContext) -> Result<ActorResponse, ActorError> {
        let actors = context
            .with_db(|conn| ActorRepo::new(conn).list())
            .map_err(ActorError::Database)?;
        Ok(ActorResponse::Listed(actors))
    }
}
