use chrono::Utc;

use crate::*;

pub struct ActorService;

impl ActorService {
    pub fn create(
        ctx: &SystemContext,
        tenant_id: String,
        name: String,
    ) -> Result<ActorResponse, ActorError> {
        let actor = Actor {
            id: ActorId::new(),
            tenant_id,
            name: ActorName::new(name),
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit(ActorEvents::ActorCreated(actor.clone()));
        Ok(ActorResponse::Created(actor))
    }

    pub fn get(ctx: &SystemContext, id: &str) -> Result<ActorResponse, ActorError> {
        let actor = ctx
            .with_db(|conn| ActorRepo::new(conn).get(id))
            .map_err(ActorError::Database)?
            .ok_or_else(|| ActorError::NotFound(id.to_string()))?;
        Ok(ActorResponse::Found(actor))
    }

    pub fn list(ctx: &SystemContext) -> Result<ActorResponse, ActorError> {
        let actors = ctx
            .with_db(|conn| ActorRepo::new(conn).list())
            .map_err(ActorError::Database)?;
        Ok(ActorResponse::Listed(actors))
    }
}
