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
            tenant_id,
            name,
            created_at: Timestamp::now(),
        };

        context
            .emit(ActorEvents::ActorCreated(actor.clone()))
            .await?;
        Ok(ActorResponse::Created(actor))
    }

    pub fn get(context: &SystemContext, id: &ActorId) -> Result<ActorResponse, ActorError> {
        let actor = ActorRepo::new(&context.db()?)
            .get(id)?
            .ok_or_else(|| ActorError::NotFound(*id))?;
        Ok(ActorResponse::Found(actor))
    }

    pub fn list(context: &SystemContext) -> Result<ActorResponse, ActorError> {
        let actors = ActorRepo::new(&context.db()?).list()?;
        Ok(ActorResponse::Listed(actors))
    }
}
