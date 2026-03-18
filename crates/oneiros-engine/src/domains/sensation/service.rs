use crate::contexts::ProjectContext;

use super::errors::SensationError;
use super::model::Sensation;
use super::repo::SensationRepo;
use super::responses::SensationResponse;

pub struct SensationService;

impl SensationService {
    pub fn set(
        ctx: &ProjectContext,
        sensation: Sensation,
    ) -> Result<SensationResponse, SensationError> {
        ctx.emit("sensation-set", &sensation);
        Ok(SensationResponse::Set(sensation))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<SensationResponse, SensationError> {
        let sensation = ctx
            .with_db(|conn| SensationRepo::new(conn).get(name))
            .map_err(SensationError::Database)?
            .ok_or_else(|| SensationError::NotFound(name.to_string()))?;
        Ok(SensationResponse::Found(sensation))
    }

    pub fn list(ctx: &ProjectContext) -> Result<SensationResponse, SensationError> {
        let sensations = ctx
            .with_db(|conn| SensationRepo::new(conn).list())
            .map_err(SensationError::Database)?;
        Ok(SensationResponse::Listed(sensations))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<SensationResponse, SensationError> {
        ctx.emit("sensation-removed", &serde_json::json!({ "name": name }));
        Ok(SensationResponse::Removed)
    }
}
