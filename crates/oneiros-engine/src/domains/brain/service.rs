use crate::*;

pub struct BrainService;

impl BrainService {
    pub async fn create(
        context: &SystemContext,
        CreateBrain { name }: &CreateBrain,
    ) -> Result<BrainResponse, BrainError> {
        let already_exists = BrainRepo::new(context).name_exists(name).await?;

        if already_exists {
            return Err(BrainError::Conflict(name.clone()));
        }

        let brain = Brain::Current(Brain::build_v1().name(name.clone()).build());

        context
            .emit(BrainEvents::BrainCreated(brain.clone()))
            .await?;

        let ref_token = RefToken::new(Ref::brain(brain.id()));
        Ok(BrainResponse::Created(
            Response::new(brain).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetBrain,
    ) -> Result<BrainResponse, BrainError> {
        let repo = BrainRepo::new(context);
        let brain = match &selector.key {
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
        let ref_token = RefToken::new(Ref::brain(brain.id()));
        Ok(BrainResponse::Found(
            Response::new(brain).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &SystemContext,
        ListBrains { filters }: &ListBrains,
    ) -> Result<BrainResponse, BrainError> {
        let listed = BrainRepo::new(context).list(filters).await?;
        Ok(BrainResponse::Listed(listed.map(|b| {
            let ref_token = RefToken::new(Ref::brain(b.id()));
            Response::new(b).with_ref_token(ref_token)
        })))
    }
}
