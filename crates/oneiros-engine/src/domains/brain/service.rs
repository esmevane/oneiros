use crate::*;

pub struct BrainService;

impl BrainService {
    pub async fn create(
        context: &SystemContext,
        name: BrainName,
    ) -> Result<BrainResponse, BrainError> {
        let already_exists = context.with_db(|conn| BrainRepo::new(conn).name_exists(&name))?;

        if already_exists {
            return Err(BrainError::Conflict(name));
        }

        let brain = Brain::builder().name(name).build();

        context
            .emit(BrainEvents::BrainCreated(brain.clone()))
            .await?;

        Ok(BrainResponse::Created(brain))
    }

    pub fn get(context: &SystemContext, name: &BrainName) -> Result<BrainResponse, BrainError> {
        let brain = context
            .with_db(|conn| BrainRepo::new(conn).get(name))?
            .ok_or_else(|| BrainError::NotFound(name.clone()))?;
        Ok(BrainResponse::Found(brain))
    }

    pub fn list(context: &SystemContext) -> Result<BrainResponse, BrainError> {
        let brains = context.with_db(|conn| BrainRepo::new(conn).list())?;
        Ok(BrainResponse::Listed(brains))
    }
}
