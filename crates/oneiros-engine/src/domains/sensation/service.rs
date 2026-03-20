use crate::*;

pub struct SensationService;

impl SensationService {
    pub fn set(
        context: &ProjectContext,
        sensation: Sensation,
    ) -> Result<SensationResponse, SensationError> {
        let name = sensation.name.clone();
        context.emit(SensationEvents::SensationSet(sensation));
        Ok(SensationResponse::SensationSet(name))
    }

    pub fn get(
        context: &ProjectContext,
        name: &SensationName,
    ) -> Result<SensationResponse, SensationError> {
        let sensation = context
            .with_db(|conn| SensationRepo::new(conn).get(name))
            .map_err(SensationError::Database)?
            .ok_or_else(|| SensationError::NotFound(name.clone()))?;
        Ok(SensationResponse::SensationDetails(sensation))
    }

    pub fn list(context: &ProjectContext) -> Result<SensationResponse, SensationError> {
        let sensations = context
            .with_db(|conn| SensationRepo::new(conn).list())
            .map_err(SensationError::Database)?;
        if sensations.is_empty() {
            Ok(SensationResponse::NoSensations)
        } else {
            Ok(SensationResponse::Sensations(sensations))
        }
    }

    pub fn remove(
        context: &ProjectContext,
        name: &SensationName,
    ) -> Result<SensationResponse, SensationError> {
        context.emit(SensationEvents::SensationRemoved(SensationRemoved {
            name: name.clone(),
        }));
        Ok(SensationResponse::SensationRemoved(name.clone()))
    }
}
