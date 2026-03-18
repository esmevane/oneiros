use chrono::Utc;
use uuid::Uuid;

use crate::contexts::SystemContext;

use super::errors::ActorError;
use super::model::Actor;
use super::repo::ActorRepo;
use super::responses::ActorResponse;

pub struct ActorService;

impl ActorService {
    pub fn create(
        ctx: &SystemContext,
        tenant_id: String,
        name: String,
    ) -> Result<ActorResponse, ActorError> {
        let actor = Actor {
            id: Uuid::now_v7().to_string(),
            tenant_id,
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("actor-created", &actor);
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
