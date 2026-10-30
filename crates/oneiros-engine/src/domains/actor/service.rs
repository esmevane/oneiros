use crate::*;

pub(crate) struct ActorService;

impl ActorService {
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateActor,
    ) -> Result<ActorResponse, ActorError> {
        let CreateActor::V1(create) = request;

        let actor = Actor::builder()
            .tenant_id(create.tenant_id)
            .name(create.name.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Actor(ActorEvents::ActorCreated(
                ActorCreated::builder_v1()
                    .actor(actor.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(ActorResponse::Created(
            ActorCreatedResponse::builder_v1()
                .actor(actor)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
        scope: &Scope<AtHost>,
        request: &GetActor,
    ) -> Result<ActorResponse, ActorError> {
        let GetActor::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let actor = ActorRepo::new(scope)
            .fetch(id)
            .await?
            .ok_or(ActorError::NotFound(id))?;
        Ok(ActorResponse::Found(
            ActorFoundResponse::builder_v1().actor(actor).build().into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        request: &ListActors,
    ) -> Result<ActorResponse, ActorError> {
        let ListActors::V1(listing) = request;
        let listed = ActorRepo::new(scope).list(&listing.filters).await?;
        Ok(ActorResponse::Listed(
            ActorsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }
}
