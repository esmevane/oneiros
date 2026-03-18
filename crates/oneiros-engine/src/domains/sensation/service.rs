use crate::*;

pub struct SensationService;

impl SensationService {
    pub fn set(
        ctx: &ProjectContext,
        sensation: Sensation,
    ) -> Result<SensationResponse, SensationError> {
        ctx.emit(SensationEvents::SensationSet(sensation.clone()));
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
        ctx.emit(SensationEvents::SensationRemoved(SensationRemoved {
            name: SensationName::new(name),
        }));
        Ok(SensationResponse::Removed)
    }
}
