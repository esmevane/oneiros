use chrono::Utc;

use crate::*;

pub struct BrainService;

impl BrainService {
    pub fn create(context: &SystemContext, name: BrainName) -> Result<BrainResponse, BrainError> {
        let already_exists = context
            .with_db(|conn| BrainRepo::new(conn).name_exists(&name))
            .map_err(BrainError::Database)?;

        if already_exists {
            return Err(BrainError::Conflict(name));
        }

        let brain = Brain {
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        context.emit(BrainEvents::BrainCreated(brain.clone()));
        Ok(BrainResponse::Created(brain))
    }

    pub fn get(context: &SystemContext, name: &BrainName) -> Result<BrainResponse, BrainError> {
        let brain = context
            .with_db(|conn| BrainRepo::new(conn).get(name))
            .map_err(BrainError::Database)?
            .ok_or_else(|| BrainError::NotFound(name.clone()))?;
        Ok(BrainResponse::Found(brain))
    }

    pub fn list(context: &SystemContext) -> Result<BrainResponse, BrainError> {
        let brains = context
            .with_db(|conn| BrainRepo::new(conn).list())
            .map_err(BrainError::Database)?;
        Ok(BrainResponse::Listed(brains))
    }
}
