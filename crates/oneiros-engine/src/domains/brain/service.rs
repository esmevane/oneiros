use chrono::Utc;

use crate::contexts::SystemContext;

use super::errors::BrainError;
use super::model::Brain;
use super::repo::BrainRepo;
use super::responses::BrainResponse;

pub struct BrainService;

impl BrainService {
    pub fn create(ctx: &SystemContext, name: String) -> Result<BrainResponse, BrainError> {
        let already_exists = ctx
            .with_db(|conn| BrainRepo::new(conn).name_exists(&name))
            .map_err(BrainError::Database)?;

        if already_exists {
            return Err(BrainError::Conflict(name));
        }

        let brain = Brain {
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("brain-created", &brain);
        Ok(BrainResponse::Created(brain))
    }

    pub fn get(ctx: &SystemContext, name: &str) -> Result<BrainResponse, BrainError> {
        let brain = ctx
            .with_db(|conn| BrainRepo::new(conn).get(name))
            .map_err(BrainError::Database)?
            .ok_or_else(|| BrainError::NotFound(name.to_string()))?;
        Ok(BrainResponse::Found(brain))
    }

    pub fn list(ctx: &SystemContext) -> Result<BrainResponse, BrainError> {
        let brains = ctx
            .with_db(|conn| BrainRepo::new(conn).list())
            .map_err(BrainError::Database)?;
        Ok(BrainResponse::Listed(brains))
    }
}
