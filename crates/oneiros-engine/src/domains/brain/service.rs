use crate::*;

pub(crate) struct BrainService;

impl BrainService {
    pub(crate) async fn create(
        context: &SystemContext,
        CreateBrain { name }: &CreateBrain,
    ) -> Result<BrainResponse, BrainError> {
        let already_exists = BrainRepo::new(context).name_exists(name).await?;

        if already_exists {
            return Err(BrainError::Conflict(name.clone()));
        }

        let brain = Brain::builder().name(name.clone()).build();

        context
            .emit(BrainEvents::BrainCreated(brain.clone()))
            .await?;

        let ref_token = RefToken::new(Ref::brain(brain.id));
        Ok(BrainResponse::Created(
            Response::new(brain).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn get(
        context: &SystemContext,
        selector: &GetBrain,
    ) -> Result<BrainResponse, BrainError> {
        let brain = BrainRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| BrainError::NotFound(selector.name.clone()))?;
        let ref_token = RefToken::new(Ref::brain(brain.id));
        Ok(BrainResponse::Found(
            Response::new(brain).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn list(
        context: &SystemContext,
        ListBrains { filters }: &ListBrains,
    ) -> Result<BrainResponse, BrainError> {
        let listed = BrainRepo::new(context).list(filters).await?;
        Ok(BrainResponse::Listed(listed.map(|b| {
            let ref_token = RefToken::new(Ref::brain(b.id));
            Response::new(b).with_ref_token(ref_token)
        })))
    }
}
