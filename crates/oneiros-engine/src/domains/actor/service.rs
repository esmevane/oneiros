use crate::*;

pub struct ActorService;

impl ActorService {
    pub async fn create(
        context: &HostLog,
        request: &CreateActor,
    ) -> Result<ActorResponse, ActorError> {
        let CreateActor::V1(create) = request;

        let actor = Actor::builder()
            .tenant_id(create.tenant_id)
            .name(create.name.clone())
            .build();

        context
            .emit(ActorEvents::ActorCreated(
                ActorCreated::builder_v1()
                    .actor(actor.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(ActorResponse::Created(
            ActorCreatedResponse::builder_v1()
                .actor(actor)
                .build()
                .into(),
        ))
    }

    pub async fn get(context: &HostLog, request: &GetActor) -> Result<ActorResponse, ActorError> {
        let GetActor::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let actor = ActorRepo::new(context.scope()?)
            .fetch(id)
            .await?
            .ok_or(ActorError::NotFound(id))?;
        Ok(ActorResponse::Found(
            ActorFoundResponse::builder_v1().actor(actor).build().into(),
        ))
    }

    pub async fn list(
        context: &HostLog,
        request: &ListActors,
    ) -> Result<ActorResponse, ActorError> {
        let ListActors::V1(listing) = request;
        let listed = ActorRepo::new(context.scope()?)
            .list(&listing.filters)
            .await?;
        Ok(ActorResponse::Listed(
            ActorsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }
}
