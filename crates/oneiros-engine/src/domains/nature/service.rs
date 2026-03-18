use crate::*;

pub struct NatureService;

impl NatureService {
    pub fn set(ctx: &ProjectContext, nature: Nature) -> Result<NatureResponse, NatureError> {
        let name = nature.name.clone();
        ctx.emit(NatureEvents::NatureSet(nature));
        Ok(NatureResponse::NatureSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<NatureResponse, NatureError> {
        let nature = ctx
            .with_db(|conn| NatureRepo::new(conn).get(name))
            .map_err(NatureError::Database)?
            .ok_or_else(|| NatureError::NotFound(name.to_string()))?;
        Ok(NatureResponse::NatureDetails(nature))
    }

    pub fn list(ctx: &ProjectContext) -> Result<NatureResponse, NatureError> {
        let natures = ctx
            .with_db(|conn| NatureRepo::new(conn).list())
            .map_err(NatureError::Database)?;
        if natures.is_empty() {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(natures))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<NatureResponse, NatureError> {
        let nature_name = NatureName::new(name);
        ctx.emit(NatureEvents::NatureRemoved(NatureRemoved {
            name: nature_name.clone(),
        }));
        Ok(NatureResponse::NatureRemoved(nature_name))
    }
}
