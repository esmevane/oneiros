use crate::*;

pub struct BrainService;

impl BrainService {
    pub async fn create(
        context: &SystemContext,
        name: BrainName,
    ) -> Result<BrainResponse, BrainError> {
        let already_exists = BrainRepo::new(context).name_exists(&name).await?;

        if already_exists {
            return Err(BrainError::Conflict(name));
        }

        let brain = Brain::builder().name(name).build();

        context
            .emit(BrainEvents::BrainCreated(brain.clone()))
            .await?;

        Ok(BrainResponse::Created(brain))
    }

    pub async fn get(
        context: &SystemContext,
        name: &BrainName,
    ) -> Result<BrainResponse, BrainError> {
        let brain = BrainRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| BrainError::NotFound(name.clone()))?;
        Ok(BrainResponse::Found(brain))
    }

    pub async fn list(context: &SystemContext) -> Result<BrainResponse, BrainError> {
        let brains = BrainRepo::new(context).list().await?;
        Ok(BrainResponse::Listed(brains))
    }
}
