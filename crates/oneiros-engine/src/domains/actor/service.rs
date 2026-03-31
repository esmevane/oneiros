use crate::*;

pub struct ActorService;

impl ActorService {
    pub async fn create(
        context: &SystemContext,
        CreateActor { tenant_id, name }: &CreateActor,
    ) -> Result<ActorResponse, ActorError> {
        let actor = Actor::builder()
            .tenant_id(*tenant_id)
            .name(name.clone())
            .build();

        context
            .emit(ActorEvents::ActorCreated(actor.clone()))
            .await?;
        Ok(ActorResponse::Created(actor))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetActor,
    ) -> Result<ActorResponse, ActorError> {
        let actor = ActorRepo::new(context)
            .get(selector.id)
            .await?
            .ok_or_else(|| ActorError::NotFound(selector.id))?;
        Ok(ActorResponse::Found(actor))
    }

    pub async fn list(context: &SystemContext) -> Result<ActorResponse, ActorError> {
        let actors = ActorRepo::new(context).list().await?;
        Ok(ActorResponse::Listed(actors))
    }
}
