use crate::*;

pub(crate) struct ActorService;

impl ActorService {
    pub(crate) async fn create(
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
        let ref_token = RefToken::new(Ref::actor(actor.id));
        Ok(ActorResponse::Created(
            Response::new(actor).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn get(
        context: &SystemContext,
        selector: &GetActor,
    ) -> Result<ActorResponse, ActorError> {
        let actor = ActorRepo::new(context)
            .get(selector.id)
            .await?
            .ok_or_else(|| ActorError::NotFound(selector.id))?;
        let ref_token = RefToken::new(Ref::actor(actor.id));
        Ok(ActorResponse::Found(
            Response::new(actor).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn list(
        context: &SystemContext,
        ListActors { filters }: &ListActors,
    ) -> Result<ActorResponse, ActorError> {
        let listed = ActorRepo::new(context).list(filters).await?;
        Ok(ActorResponse::Listed(listed.map(|a| {
            let ref_token = RefToken::new(Ref::actor(a.id));
            Response::new(a).with_ref_token(ref_token)
        })))
    }
}
