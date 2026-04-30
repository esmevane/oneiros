use crate::*;

pub struct BrainService;

impl BrainService {
    pub async fn create(
        context: &HostLog,
        request: &CreateBrain,
    ) -> Result<BrainResponse, BrainError> {
        let CreateBrain::V1(create) = request;
        let already_exists = BrainRepo::new(context.scope()?)
            .name_exists(&create.name)
            .await?;

        if already_exists {
            return Err(BrainError::Conflict(create.name.clone()));
        }

        let brain = Brain::builder().name(create.name.clone()).build();

        context
            .emit(BrainEvents::BrainCreated(
                BrainCreated::builder_v1()
                    .brain(brain.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(BrainResponse::Created(
            BrainCreatedResponse::builder_v1()
                .brain(brain)
                .build()
                .into(),
        ))
    }

    pub async fn get(context: &HostLog, request: &GetBrain) -> Result<BrainResponse, BrainError> {
        let GetBrain::V1(lookup) = request;
        let repo = BrainRepo::new(context.scope()?);
        let brain = match &lookup.key {
            ResourceKey::Key(name) => repo
                .get(name)
                .await?
                .ok_or_else(|| BrainError::NotFound(name.clone()))?,
            ResourceKey::Ref(token) => {
                let Ref::V0(resource) = token.inner().clone();
                match resource {
                    Resource::Brain(id) => repo
                        .get_by_id(&id)
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
        context: &HostLog,
        request: &ListBrains,
    ) -> Result<BrainResponse, BrainError> {
        let ListBrains::V1(listing) = request;
        let listed = BrainRepo::new(context.scope()?)
            .list(&listing.filters)
            .await?;
        Ok(BrainResponse::Listed(
            BrainsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }
}
