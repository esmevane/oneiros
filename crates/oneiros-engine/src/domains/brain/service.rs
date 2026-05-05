use crate::*;

pub struct BrainService;

impl BrainService {
    pub async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateBrain,
    ) -> Result<BrainResponse, BrainError> {
        let CreateBrain::V1(create) = request;
        let already_exists = BrainRepo::new(scope).name_exists(&create.name).await?;

        if already_exists {
            return Err(BrainError::Conflict(create.name.clone()));
        }

        let brain = Brain::builder().name(create.name.clone()).build();
        let name = brain.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Brain(BrainEvents::BrainCreated(
                BrainCreated::builder_v1().brain(brain).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = BrainRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(BrainError::NotFound(name))?;

        Ok(BrainResponse::Created(
            BrainCreatedResponse::builder_v1()
                .brain(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtHost>,
        request: &GetBrain,
    ) -> Result<BrainResponse, BrainError> {
        let GetBrain::V1(lookup) = request;
        let repo = BrainRepo::new(scope);
        let brain = match &lookup.key {
            ResourceKey::Key(name) => repo
                .fetch(name)
                .await?
                .ok_or_else(|| BrainError::NotFound(name.clone()))?,
            ResourceKey::Ref(token) => {
                let Ref::V0(resource) = token.inner().clone();
                match resource {
                    Resource::Brain(id) => repo
                        .fetch_by_id(&id)
                        .await?
                        .ok_or(BrainError::NotFoundById(id))?,
                    other => {
                        return Err(BrainError::Resolve(ResolveError::WrongKind {
                            expected: "brain",
                            got: other.label(),
                        }));
                    }
                }
            }
        };
        Ok(BrainResponse::Found(
            BrainFoundResponse::builder_v1().brain(brain).build().into(),
        ))
    }

    pub async fn list(
        scope: &Scope<AtHost>,
        request: &ListBrains,
    ) -> Result<BrainResponse, BrainError> {
        let ListBrains::V1(listing) = request;
        let listed = BrainRepo::new(scope).list(&listing.filters).await?;
        Ok(BrainResponse::Listed(
            BrainsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }
}
