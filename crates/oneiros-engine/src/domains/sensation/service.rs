use crate::*;

pub struct SensationService;

impl SensationService {
    pub fn set(
        ctx: &ProjectContext,
        sensation: Sensation,
    ) -> Result<SensationResponse, SensationError> {
        let name = sensation.name.clone();
        ctx.emit(SensationEvents::SensationSet(sensation));
        Ok(SensationResponse::SensationSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<SensationResponse, SensationError> {
        let sensation = ctx
            .with_db(|conn| SensationRepo::new(conn).get(name))
            .map_err(SensationError::Database)?
            .ok_or_else(|| SensationError::NotFound(name.to_string()))?;
        Ok(SensationResponse::SensationDetails(sensation))
    }

    pub fn list(ctx: &ProjectContext) -> Result<SensationResponse, SensationError> {
        let sensations = ctx
            .with_db(|conn| SensationRepo::new(conn).list())
            .map_err(SensationError::Database)?;
        if sensations.is_empty() {
            Ok(SensationResponse::NoSensations)
        } else {
            Ok(SensationResponse::Sensations(sensations))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<SensationResponse, SensationError> {
        let sensation_name = SensationName::new(name);
        ctx.emit(SensationEvents::SensationRemoved(SensationRemoved {
            name: sensation_name.clone(),
        }));
        Ok(SensationResponse::SensationRemoved(sensation_name))
    }
}
