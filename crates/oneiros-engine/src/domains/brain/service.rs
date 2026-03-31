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

        let brain = Brain::builder().name(name.clone()).build();

        context
            .emit(BrainEvents::BrainCreated(brain.clone()))
            .await?;

        Ok(BrainResponse::Created(brain))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetBrain,
    ) -> Result<BrainResponse, BrainError> {
        let brain = BrainRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| BrainError::NotFound(selector.name.clone()))?;
        Ok(BrainResponse::Found(brain))
    }

    pub async fn list(context: &SystemContext) -> Result<BrainResponse, BrainError> {
        let brains = BrainRepo::new(context).list().await?;
        Ok(BrainResponse::Listed(brains))
    }
}
