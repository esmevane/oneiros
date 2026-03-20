use crate::*;

pub struct NatureService;

impl NatureService {
    pub fn set(context: &ProjectContext, nature: Nature) -> Result<NatureResponse, NatureError> {
        let name = nature.name.clone();
        context.emit(NatureEvents::NatureSet(nature));
        Ok(NatureResponse::NatureSet(name))
    }

    pub fn get(context: &ProjectContext, name: &NatureName) -> Result<NatureResponse, NatureError> {
        let nature = context
            .with_db(|conn| NatureRepo::new(conn).get(name))?
            .ok_or_else(|| NatureError::NotFound(name.clone()))?;
        Ok(NatureResponse::NatureDetails(nature))
    }

    pub fn list(context: &ProjectContext) -> Result<NatureResponse, NatureError> {
        let natures = context
            .with_db(|conn| NatureRepo::new(conn).list())
            .map_err(NatureError::Database)?;
        if natures.is_empty() {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(natures))
        }
    }

    pub fn remove(
        context: &ProjectContext,
        name: &NatureName,
    ) -> Result<NatureResponse, NatureError> {
        context.emit(NatureEvents::NatureRemoved(NatureRemoved {
            name: name.clone(),
        }));
        Ok(NatureResponse::NatureRemoved(name.clone()))
    }
}
